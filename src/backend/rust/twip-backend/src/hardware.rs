use crate::TripData;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

// Struct which is used for sending hardware values to the frontend
// For now, the distance, speed and pace of the ride is send to the user.
#[derive(Serialize, Deserialize)]
pub struct HardwareData {
    pub distance: usize,
    pub speed: usize,
    pub pace: usize,
}

// Function which creates a new instance of HardwareData, using a distance, speed and pace variable.
impl HardwareData {
    pub fn new(distance: usize, speed: usize, pace: usize) -> HardwareData {
        HardwareData {
            distance: distance,
            speed: speed,
            pace: pace,
        }
    }
}

// Endpoint which sends the speed, pace and distance of the current trip.
#[get("/hardware/data")]
pub fn generate_hardware_data(trip_data: State<TripData>) -> Json<HardwareData> {
    let data = HardwareData::new(
        trip_data.distance.load(Ordering::Relaxed),
        trip_data.get_last_speed_value(),
        trip_data.pace.load(Ordering::Relaxed),
    );
    Json(data)
}

// Endpoint which the hardware uses to send the speed to the backend. The backend then updates the trip stats accordingly.
#[get("/hardware/speed/<speed>")]
pub fn handle_speed_incomming(speed: usize, trip_data: State<TripData>) -> String {
    trip_data.add_speed(speed);
    trip_data.update_values();
    format!("Added value '{}'", speed)
}

// Testing of hardware.rs
#[cfg(test)]
mod tests {
    use super::HardwareData;
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    // Testing whether the endpoint contains "distance", "speed" and "pace" and wheter the endpoint returns an Ok as answer.
    #[test]
    fn test_generate_hardware_data() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hardware/data").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let message = response.body_string().unwrap();
        assert_eq!(message.contains("distance"), true);
        assert_eq!(message.contains("speed"), true);
        assert_eq!(message.contains("pace"), true);
    }

    // Test which sees if the HardwareData got created correctly
    #[test]
    fn test_hardware_data_new() {
        let data = HardwareData::new(50, 15, 30);
        assert_eq!(data.distance, 50);
        assert_eq!(data.speed, 15);
        assert_eq!(data.pace, 30);
    }
}
