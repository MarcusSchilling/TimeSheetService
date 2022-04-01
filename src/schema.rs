table! {
    timesheets (id) {
        id -> Int4,
        name -> Varchar,
        time_done -> Interval,
        time_target -> Interval,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
        grade -> Nullable<Float4>,
        ects -> Nullable<Float4>,
    }
}
