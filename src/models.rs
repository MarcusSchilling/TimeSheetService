use diesel::pg::types::date_and_time::{PgInterval, PgTimestamp};

#[derive(Queryable)]
pub struct Timesheet {
    pub id: i32,
    pub name: String,
    pub time_done: PgInterval,
    pub time_target: PgInterval,
    pub start_date: Option<PgTimestamp>,
    pub end_date: Option<PgTimestamp>,
    pub grade: Option<f32>,
    pub ects: Option<f32>
}
use super::schema::timesheets;

#[derive(Insertable)]
#[table_name="timesheets"]
pub struct NewTimesheet<'a> {
    pub name: &'a str,
    pub time_done: &'a PgInterval,
    pub time_target: &'a PgInterval
}

#[derive(AsChangeset)]
#[table_name="timesheets"]
pub struct TimesheetForm<'a> {
    pub start_date: Option<&'a PgTimestamp>,
    pub end_date: Option<&'a PgTimestamp>,
    pub grade: Option<& 'a f32>,
    pub ects: Option<& 'a f32>,
}