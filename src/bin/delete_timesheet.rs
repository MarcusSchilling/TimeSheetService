extern crate diesel;

use timesheet_service::*;
use std::env::args;
use timesheet_service::establish_connection;

fn main() {

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = delete_timesheet_by_name(&connection, pattern);

    println!("Deleted {} timesheets", num_deleted);
}