extern crate diesel;

use timesheet_service::*;
use timesheet_service::models::*;
use self::diesel::prelude::*;
use timesheet_service::establish_connection;

fn main() {
    use timesheet_service::schema::timesheets::dsl::*;

    let connection = establish_connection();
    let results = timesheets
        .limit(5)
        .load::<Timesheet>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} timesheets", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
    }
}