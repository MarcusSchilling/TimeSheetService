extern crate diesel;

use std::io::{stdin, Read};
use timesheet_service::{establish_connection, create_timesheet};
use diesel::pg::types::date_and_time::PgInterval;

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    let post = create_timesheet(&connection, name, &PgInterval{days: 0, microseconds: 0, months: 0}, &PgInterval{days: 1, microseconds: 0, months: 0});
    println!("\nSaved draft {} with id {}", name, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";