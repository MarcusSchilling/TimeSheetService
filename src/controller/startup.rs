use crate::controller::hello::static_rocket_route_info_for_hello;

pub fn startup() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}
