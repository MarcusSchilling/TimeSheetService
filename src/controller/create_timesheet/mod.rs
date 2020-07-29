use timesheet_service::models::{Timesheet, TimesheetDTO};
use std::io::{self, Write};
use rocket_contrib::json::Json;
use rocket::response::status::Forbidden;

#[post("/newTimesheet", format = "application/json", data = "<timesheet>")]
pub fn new_timesheet(timesheet: Json<TimesheetDTO>) -> Json<TimesheetDTO> {
    print!("{}",timesheet.name);
    print!("{}",timesheet.id);
    return timesheet;
}
