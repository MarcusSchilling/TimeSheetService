mod hello;
use crate::controller::startup::hello::static_rocket_route_info_for_hello;

pub fn startup() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}
