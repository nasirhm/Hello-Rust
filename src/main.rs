#![feature(proc_macro_hygiene, decl_macro)] // Nightly-only language needed for rocket.
// Import OpenAPI macros
#[macro_use]
extern crate rocket_okapi;

use rocket_okapi::JsonSchema;
use rocket_contrib::json::Json;
use serde::*;
/// Host information structure returned at /hostinfo
#[derive(Serialize, JsonSchema, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

// Import the rocker macros
#[macro_use]
extern crate rocket;

/// Create route / that returns "Hello, World!"
// Adding the route to OpenAPI schema.
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
use rocket_okapi::{OpenApiError, Result};
// Adding the route to OpenAPI schema.
#[openapi]
#[get("/hostinfo")]
fn hostinfo() -> Result<Json<HostInfo>> {
    match gethostname::gethostname().into_string() {
        Ok(hostname) => Ok(Json(HostInfo {
            hostname: hostname,
            pid: std::process::id(),
            uptime: psutil::host::uptime().unwrap().as_secs(),
        })),
    Err(_) => Err(OpenApiError::new(format!(
                "hotname does not parse a UTF-8"
                ))),
    }
}

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index, hostinfo])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
            url: Some("../openapi.json".to_owned()),
            urls: None,}),
            )
        .launch();
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
