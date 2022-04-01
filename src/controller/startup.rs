use crate::controller::hello::static_rocket_route_info_for_hello;
use crate::controller::create_timesheet::static_rocket_route_info_for_new_timesheet;

pub fn startup() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, new_timesheet])
}
