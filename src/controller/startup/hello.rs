
#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}


#[cfg(test)]
mod test {
    use crate::controller::startup::startup;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(startup()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}