#![feature(proc_macro_hygiene, decl_macro)] //Nightly-only language needed for rocket.
use rocket_contrib::json::Json;
use serde::*;
/// Host information structure returned at /hostinfo
#[derive(Serialize, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

// Import the rocker macros
#[macro_use]
extern crate rocket;

/// Create route / that returns "Hello, World!"
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hostinfo")]
fn hostinfo() -> Json<HostInfo> {
    // gets the current machine hostname or "unknown" if the hostname doesn't
    // parse into UTF-8 (very unlikely)
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap();

    Json(HostInfo{
        hostname: hostname,
        pid: std::process::id(),
        uptime: psutil::host::uptime()
            .unwrap() // normally this is a bad idea, but this code is
                      // very unlikely to fail.
            .as_secs(),
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index, hostinfo]).launch();
}

#[cfg(test)] // Only compile this when unit testing is requested
mod tests {
    use super::*; // Modules are their own scope, so you
                  // need to explicitly use the stuff in
                  // the parent module.
    use rocket::http::Status;
    use rocket::local::*;
    #[test]
    fn test_inded(){
        // create the rocket instance to test
        let rkt = rocket::ignite().mount("/", routes![index]);

        // create a HTTP client bount to this rocket instance
        let client = Client::new(rkt).expect("valid rocket");

        // get a HTTP response
        let mut response = client.get("/").dispatch();

        // Ensure it returns HTTP 200
        assert_eq!(response.status(), Status::Ok);

        // Ensure the body is what we expect it to be
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

}
