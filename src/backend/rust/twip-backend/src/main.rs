#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rocket_sync_db_pools;

mod hardware;
mod trip;
mod users;

use rocket_sync_db_pools::database;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use chrono::offset::Local;

use trip::TripData;
use users::LoggedUser;

use postgres::{Client, Error, NoTls};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

// Struct which rocket requires to connect to the database.
#[database("postgres")]
struct PostgresConn(postgres::Client);

// Function which creates a client which can interact with the database.
fn create_client() -> Result<Client, Error> {
    Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)
}

// Function which creates an instance of the backend, which manages the calling of endpoints.
fn rocket() -> rocket::Rocket {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let trip_data = TripData {
        distance: AtomicUsize::new(0),
        speed_values: Arc::new(Mutex::new(Vec::new())),
        pace: AtomicUsize::new(0),
        start_time: Mutex::new(Local::now()),
    };

    let user = LoggedUser::new(Arc::new(Mutex::new(String::from("none"))));

    trip_data.add_speed(0);

    // Attach all endpoints and keep track of two states.
    rocket::ignite()
        .manage(trip_data)
        .manage(user)
        .mount(
            "/",
            routes![
                hardware::generate_hardware_data,
                hardware::handle_speed_incomming,
                trip::handle_trip_start,
                trip::handle_trip_end,
                trip::get_last_trip,
                trip::delete_trip_history,
                users::logout_user,
                users::authenticate_user,
                users::register_user,
                users::delete_user,
                users::security_question_auth,
                users::change_password,
            ],
        )
        .attach(cors.to_cors().unwrap())
}

// Launch the rocket!
fn main() {
    rocket().launch();
}

// Tests for main.rs
#[cfg(test)]
mod tests {
    use super::rocket;
    use super::create_client;
    use std::time::Duration;

    #[test]
    fn test_rocket() {
        let rocket = rocket();

        // 12 routes and one CORS route.
        assert_eq!(rocket.routes().count(), 13);
    }

    #[test]
    fn test_create_client() {
        let mut client = create_client().unwrap();

        client.is_valid(Duration::from_secs(1)).expect("Test failed because connection to database failed");
    }
}
