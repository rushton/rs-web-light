extern crate iron;
extern crate router;
extern crate serde_json;
extern crate serde;


#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::Header;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", status, "status");
    router.put("/led/:index/:red/:green/:blue/:intensity", set_color, "set_color");

    Iron::new(router).http("localhost:3000").unwrap();

    println!("Hello, world!");
}

#[derive(Serialize)]
pub struct Status {
    status: String
}
fn status(_req: &mut Request) -> IronResult<Response> {
   Ok(Response::with((
       status::Ok,
       Header(headers::ContentType::json()),
       &serde_json::to_string(&Status { status: "GOOD".to_string() }).unwrap() as &str
   )))
}

fn set_color(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "GOOD")))
}
