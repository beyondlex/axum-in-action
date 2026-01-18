mod error;

use axum::extract::Query;
use axum::{Router, response::IntoResponse, routing::get};
use std::collections::HashMap;
use anyhow::anyhow;
use serde::Deserialize;
use crate::error::{Error, Result};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/res", get(res))
        ;

    let addr = "0.0.0.0:3020";
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

struct CustomResp;

impl IntoResponse for CustomResp {
    fn into_response(self) -> axum::response::Response {
        "CustomResp".into_response()
    }
}


// which calls one of these handlers
async fn root() -> &'static str {
    "Hello, World!"
}
async fn get_foo(Query(params): Query<HashMap<String, String>>) -> &'static str {
    println!("{:?}", params);
    "hello"
}
async fn post_foo() {}

async fn foo_bar() -> CustomResp {
    CustomResp
}

#[derive(Deserialize)]
struct ResParam {
    value: u8,
}

#[axum::debug_handler]
async fn res(Query(param): Query<ResParam>) -> impl IntoResponse {
    match param.value {
        1 => Ok("one".to_string()), // todo: wrapping string to common response structure
        2 => Ok(read_file()?),
        _ => Err(Error::NotFound("no one here".to_string()))
    }
}

fn read_file() -> anyhow::Result<String> {
    let config = std::fs::read_to_string("Cargo.toml2")
        .map_err(|e| anyhow!("Read config failed {}", e))?;
    Ok(config)
}
