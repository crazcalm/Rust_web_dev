#![deny(warnings)]

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
    // POST /form/  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("form"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|person: Person| warp::reply::json(&person));

    warp::serve(promote).run(([0, 0, 0, 0], 3030)).await
}
