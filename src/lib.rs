//uncomment when tests.rs should be used for unit tests
//#[cfg(test)]
//mod tests;
pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::pg::types::date_and_time::PgInterval;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Timesheet, NewTimesheet, TimesheetForm};

pub fn create_timesheet<'a>(conn: &PgConnection, name: &'a str, time_done: &'a PgInterval, time_target: &'a PgInterval) -> Timesheet {
    use schema::timesheets;

    let new_timesheet = NewTimesheet{
        name,
        time_done,
        time_target
    };
    diesel::insert_into(timesheets::table)
        .values(&new_timesheet)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_or_set_grade<'a>(conn: &PgConnection, id: &'a i32, grade: &'a f32) {
    use schema::timesheets::dsl::timesheets;

    let update_timesheet = TimesheetForm {
        start_date: None,
        end_date: None,
        grade: Option::from(grade),
        ects: None
    };
    let timesheet = diesel::update(timesheets.find(id))
        .set(&update_timesheet)
        .get_result::<Timesheet>(conn)
        .expect(&format!("Unable to find timesheet {}", id));
    println!("Updated timesheet {}", timesheet.name);

}