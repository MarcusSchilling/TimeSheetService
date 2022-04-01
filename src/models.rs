use diesel::pg::types::date_and_time::{PgInterval, PgTimestamp};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use std::fmt;

use serde::de::{self, Visitor};

mod timestamp_format {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use diesel::sql_types::Interval;

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &Interval,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize, Serialize)]
pub struct TimesheetDTO {
    pub id: i32,
    pub name: String,
    /*pub time_done: Interval,
    pub time_target: Interval,*/
    #[serde(with = "my_date_format")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub end_date: DateTime<Utc>,
    pub grade: Option<f32>,
    pub ects: Option<f32>
}

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
use diesel::sql_types::{Timestamp, Interval};

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