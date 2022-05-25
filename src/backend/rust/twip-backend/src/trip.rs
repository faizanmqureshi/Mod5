use crate::{create_client, LoggedUser};
use chrono::offset::Local;
use chrono::{DateTime, Duration};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// Struct which will be used to store the information of a trip.
pub struct TripData {
    pub distance: AtomicUsize,
    pub speed_values: Arc<Mutex<Vec<AtomicUsize>>>,
    pub pace: AtomicUsize,
    pub start_time: Mutex<DateTime<Local>>,
}

// Methods which are used for saving trip data and requesting trip data.
impl TripData {
    // Method which starts the trip and sets all the variables to zero.
    pub fn start_trip(&self) -> DateTime<Local> {
        let mut time_lock = self.start_time.lock().expect("lock on start_time");
        *time_lock = Local::now();

        self.distance.fetch_min(0, Ordering::Relaxed);
        let speed_lock = Arc::clone(&self.speed_values);

        let mut vector = speed_lock.lock().expect("lock on speed_values");
        vector.clear();
        vector.push(AtomicUsize::new(0));

        *time_lock
    }

    // Returns the distance of the current trip.
    pub fn get_distance(&self) -> usize {
        self.distance.load(Ordering::Relaxed)
    }

    // Adds a speed value to the vector which saves speed values.
    pub fn add_speed(&self, speed: usize) {
        let lock = Arc::clone(&self.speed_values);
        let mut vector = lock.lock().expect("lock on speed_values");
        vector.push(AtomicUsize::new(speed.into()));
    }

    // Gets the current speed of the trip.
    pub fn get_last_speed_value(&self) -> usize {
        let lock = Arc::clone(&self.speed_values);
        let vector = lock.lock().expect("lock on speed_values");
        let length = &vector.len();
        vector[length - 1].load(Ordering::Relaxed)
    }

    // Gets the fastest speed of the trip.
    pub fn get_top_speed(&self) -> usize {
        let lock = Arc::clone(&self.speed_values);
        let vector = lock.lock().expect("lock on speed_values");
        let mut top_speed: usize = 0;
        for speed in vector.iter() {
            let speed_usize = speed.load(Ordering::Relaxed);
            if speed_usize > top_speed {
                top_speed = speed_usize;
            }
        }
        top_speed
    }

    // Gets the current pace of the trip.
    pub fn get_pace(&self) -> usize {
        self.pace.load(Ordering::Relaxed)
    }

    // Gets the current time of the trip.
    pub fn get_start_time(&self) -> DateTime<Local> {
        let lock = self.start_time.lock().expect("lock on start_time");
        *lock
    }

    // Updates the values after a new speed value.
    pub fn update_values(&self) {
        let lock = Arc::clone(&self.speed_values);
        let vector = lock.lock().expect("lock on speed_values");
        self.distance.fetch_add(
            vector[vector.len() - 1].load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        let mut sum = 0;
        for i in vector.iter() {
            sum += i.load(Ordering::Relaxed);
        }
        self.pace.store(sum / vector.len(), Ordering::Relaxed);
    }
}

// Struct which represents the trip table from the database.
#[derive(Serialize, Deserialize)]
pub struct DatabaseTripData {
    distance: i32,
    top_speed: i32,
    pace: i32,
    trip_length: i32,
}

// Methods for DatabaseTripData struct
impl DatabaseTripData {
    // Function which creates a new DatabaseTripData struct.
    pub fn new(distance: i32, top_speed: i32, pace: i32, trip_length: i32) -> DatabaseTripData {
        DatabaseTripData {
            distance: distance,
            top_speed: top_speed,
            pace: pace,
            trip_length: trip_length,
        }
    }
}

// Endpoint which indicates that a trip must start and updates the time.
#[get("/trip/start")]
pub fn handle_trip_start(trip_data: State<TripData>) -> String {
    let time = trip_data.start_trip();
    format!("{}", time.format("%d/%m/%Y %T"))
}

// Endpoint which finishes a trip and stores it in the database.
#[post("/trip/finish")]
pub fn handle_trip_end(
    trip_data: State<TripData>,
    user: State<LoggedUser>,
) -> Json<DatabaseTripData> {
    // First, we create a client and create the values for the database.
    let mut client = create_client().unwrap();

    let start_time = trip_data.get_start_time();
    let end_time = Local::now();
    let total_duration = DateTime::signed_duration_since(end_time, start_time);

    let user = user.get_logged_user().to_string();
    let distance: i32 = trip_data.get_distance() as i32;
    let top_speed: i32 = trip_data.get_top_speed() as i32;
    let pace: i32 = trip_data.get_pace() as i32;
    let trip_length: i32 = Duration::num_seconds(&total_duration) as i32
        + (Duration::num_minutes(&total_duration) * 60) as i32
        + (Duration::num_hours(&total_duration) * 60 * 60) as i32;

    // We then execute a query which will store the trip in the database to the according user.
    client.execute(
        "INSERT INTO trip_data (distance, trip_user, top_speed, pace, trip_length) VALUES($1, $2, $3, $4, $5)",
        &[&distance, &user, &top_speed, &pace, &trip_length],
    ).expect("database interaction failed");

    Json(DatabaseTripData::new(
        distance,
        top_speed,
        pace,
        trip_length,
    ))
}

// Get the last trip made by a user from the database.
#[get("/trip/history")]
pub fn get_last_trip(user: State<LoggedUser>) -> Json<DatabaseTripData> {
    // Create a connection with the database.
    let mut client = create_client().unwrap();

    // Then we fetch the latest trip from the last user.
    let user_name = user.get_logged_user();
    let row = client.query_one(
        "SELECT trip_id, trip_user, distance, top_speed, pace, trip_length FROM trip_data WHERE trip_user = $1 ORDER BY trip_id DESC LIMIT 1",
        &[&user_name],
    ).unwrap();

    // Then we send the data back to the user.
    let distance: i32 = row.get(2);
    let top_speed: i32 = row.get(3);
    let pace: i32 = row.get(4);
    let trip_length: i32 = row.get(5);

    Json(DatabaseTripData::new(
        distance,
        top_speed,
        pace,
        trip_length,
    ))
}

// Delete all trips from the current logged in user from the database.
#[delete("/trip/history/delete")]
pub fn delete_trip_history(user: State<LoggedUser>) -> String {
    // First create a connection to the database.
    let mut client = create_client().unwrap();

    // We then delete all trips from the current logged in user from the database.
    let username = user.get_logged_user();
    client
        .execute("DELETE FROM trip_data WHERE trip_user = $1", &[&username])
        .unwrap();
    String::from("Succesful deletion")
}

#[cfg(test)]
mod tests {

    

}
