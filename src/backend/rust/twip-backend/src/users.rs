extern crate hex;
extern crate rand;
extern crate sha2;
use crate::create_client;
use rand::distributions::Alphanumeric;
use rocket::State;
use rocket_basicauth::BasicAuth;
use sha2::{Digest, Sha512};
use std::sync::{Arc, Mutex};

use rand::prelude::*;

// Struct used for keeping track of the current logged in user.
pub struct LoggedUser {
    user: Arc<Mutex<String>>,
}

// Methods which can create a new LoggedUser and methods onto them.
impl LoggedUser {
    
    //Creates a new LoggedUser.
    pub fn new(user: Arc<Mutex<String>>) -> LoggedUser {
        LoggedUser { user: user }
    }

    // Returs the logged user.
    pub fn get_logged_user(&self) -> String {
        let lock = Arc::clone(&self.user);
        let user = lock.lock().expect("lock on speed_values");
        format!("{}", *user)
    }

    // Changes the current logged user.
    pub fn set_logged_user(&self, user_name: &String) {
        let lock = Arc::clone(&self.user);
        let mut user = lock.lock().expect("lock on speed_values");
        *user = user_name.to_string()
    }
}

// Hashes a given string to a Sha512 encoded string.
fn hash_password(password: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    let res = hasher.finalize();
    format!("{:x}", res)
}

// Endpoint which logs the current user out.
#[get("/users/logout")]
pub fn logout_user(user: State<LoggedUser>) -> String {
    let logout_string = String::from("none");
    user.set_logged_user(&logout_string);
    logout_string
}

// Function which checks whether the user login was valid.
fn authentication(user_name: String, passw: String) -> &'static str {
    let user = user_name;
    let mut input = passw;

    //get hashed password and salt from database
    let mut client = create_client().unwrap();
    let data_row = match client.query_one(
        "SELECT user_password, user_salt FROM users WHERE user_name = $1",
        &[&user],
    ) {
        Ok(data) => data,
        _ => return "false",
    };
    let stored_passw: String = data_row.get(0);
    let salt: String = data_row.get(1);

    //add salt to password and hash
    input.push_str(&salt);
    let output = hash_password(&input);

    //compare hashed input and stored password hash and return result as boolean
    match output.eq(&stored_passw) {
        true => "true",
        false => "false",
    }
}

// Endpoint which checks whether the user entered a valid login.
#[get("/users/authenticate")]
pub fn authenticate_user(auth: BasicAuth, logged_user: State<LoggedUser>) -> &'static str {
    let input = auth.password;
    let user = auth.name;
    match authentication(user.clone(), input) {
        "true" => {
            logged_user.set_logged_user(&user);
            return "true";
        }
        &_ => return "false",
    }
}

// Endpoint which registers a new user.
#[post("/users/register/<security1>/<security2>")]
pub fn register_user(auth: BasicAuth, security1: String, security2: String) -> String {
    let user_name = auth.name;
    let mut passw = auth.password;
    let security_q1 = hash_password(&security1);
    let security_q2 = hash_password(&security2);

    //generate random salt
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    //hash password with salt
    passw.push_str(&salt);
    let hashed_passw = hash_password(&passw);

    //insert as new entry in database
    let mut client = create_client().unwrap();

    // Database interaction which inserts the new user.
    client
        .execute(
            "INSERT INTO users (user_name, user_password, user_salt, security_q1, security_q2) 
        VALUES($1, $2, $3, $4, $5)",
            &[&user_name, &hashed_passw, &salt, &security_q1, &security_q2],
        )
        .expect("database interaction failed");

    
    format!("true")
}

// Endpoint which deletes the current user.
#[delete("/users/delete")]
pub fn delete_user(auth: BasicAuth) -> String {
    let user_name = auth.name;
    let passw = auth.password;

    //check if password and username match database
    let permission = authentication(user_name.clone(), passw);

    //if the given name and password match, delete the user, otherwise return false
    if permission == "true" {
        let mut client = create_client().unwrap();
        client
            .execute("DELETE FROM trip_data WHERE trip_user = $1", &[&user_name])
            .unwrap();
        client
            .execute("DELETE FROM users WHERE user_name = $1", &[&user_name])
            .unwrap();

        return format!("user {} has been deleted", user_name);
    }

    if permission == "false" {
        return format!("unable to verify user {} with given password", user_name);
    }

    format!("ERROR: process failed")
}

// Endpoint which checks whether the user entered the correct security answer answers.
#[get("/users/authenticate/<security1>/<security2>")]
pub fn security_question_auth(security1: String, security2: String, auth: BasicAuth) -> String {
    let mut client = create_client().unwrap();
    print!("{}, {}, {}", &auth.name, security1, security2);
    let data_row = match client.query_one(
        "SELECT security_q1, security_q2 FROM users WHERE user_name = $1",
        &[&auth.name],
    ) {
        Ok(data) => data,
        _ => return "false".to_string(),
    };

    let stored_q1: String = data_row.get(0);
    let stored_q2: String = data_row.get(1);

    let q1 = hash_password(&security1).eq(&stored_q1);
    let q2 = hash_password(&security2).eq(&stored_q2);

    if q1 == true && q2 == true {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}

// Endpoint which changes the current users password.
#[patch("/users/change_password")]
pub fn change_password(auth: BasicAuth) -> String {
    let mut client = create_client().unwrap();
    let mut passw = auth.password;
    //generate new unique salt
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    //hash password with salt
    passw.push_str(&salt);
    let hashed_passw = hash_password(&passw);

    //insert new password and salt into database
    client
        .execute(
            "UPDATE users SET user_password = $1, user_salt = $2 WHERE user_name = $3",
            &[&hashed_passw, &salt, &auth.name],
        )
        .expect("database interaction failed");

    format!("password succesfully changed")
}
