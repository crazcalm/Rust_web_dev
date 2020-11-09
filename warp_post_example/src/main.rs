#![deny(warnings)]

use log;
use pretty_env_logger;
use serde::{Deserialize, Serialize};

use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    email: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Used to check it the server is running
    let health_check = warp::get().map(|| {
        log::info!("Server Health Check was Successful");
        "The server is up!"
    });

    // POST /form/  {"name":"Sean","rate":2}
    let form = warp::post()
        .and(warp::path("form"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|person: Person| {
            log::info!("/form was successfully reached");

            warp::reply::json(&person)
        });

    let routes = health_check.or(form);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await
}
