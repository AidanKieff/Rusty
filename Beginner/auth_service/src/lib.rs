#![allow(dead_code, unused_variables)]
use rand::prelude::*;


mod database;

mod auth_utils;

pub use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    let timeout = 
        thread_rng().gen_range(100..500);
    println!("The timeout is {}", timeout);
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
