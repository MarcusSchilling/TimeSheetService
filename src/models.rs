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