mod error;
mod response;

use axum::extract::Query;
use axum::{Router, response::IntoResponse, routing::get};
use std::collections::HashMap;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};
use crate::response::Resp;

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

#[derive(Serialize)]
struct Foo {
    id: u8,
    name: String,
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

async fn foo_bar(Query(param): Query<ResParam>) -> Result<Resp<Foo>> {
    let foo = Foo { id: 1, name: "bar".to_string() };
    if param.value < 5 {
        return Err(Error::BadRequest(String::from("bad request")));
    }
    Ok(Resp::ok(foo))
}

#[derive(Deserialize)]
struct ResParam {
    value: u8,
}

#[axum::debug_handler]
async fn res(Query(param): Query<ResParam>) -> impl IntoResponse {
    let resp = match param.value {
        1 => "one".to_string(),
        2 => read_file()?,
        _ => return Err(Error::NotFound("no one here".to_string())),
    };
    Ok(Resp::ok(resp))
}

fn read_file() -> anyhow::Result<String> {
    let config = std::fs::read_to_string("Cargo.toml")
        .map_err(|e| anyhow!("Read config failed {}", e))?;
    Ok(config)
}
