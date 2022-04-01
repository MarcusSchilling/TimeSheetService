extern crate diesel;

use self::diesel::prelude::*;
use timesheet_service::*;
use std::env::args;
use timesheet_service::{establish_connection, update_or_set_grade};
use timesheet_service::models::Timesheet;

fn main() {
    use timesheet_service::schema::timesheets::dsl::{timesheets, grade};

    let id = args().nth(1).expect("update grade requires a timesheet id")
        .parse::<i32>().expect("Invalid ID");
    let current_grade = args().nth(2).expect("you have to provide the grade to which one should update")
        .parse::<f32>().expect("Invalid Grade");
    let connection = establish_connection();
    update_or_set_grade(&connection, &id, &current_grade);
}