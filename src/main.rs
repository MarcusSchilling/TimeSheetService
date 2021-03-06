#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controller;
use crate::controller::startup::*;

fn main() {
    startup().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use crate::controller::startup::startup;

    #[test]
    fn hello_world() {
        let client = Client::new(startup()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}