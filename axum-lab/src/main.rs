#![allow(unused)]

use axum::response::{Html, IntoResponse};
use axum::{Router, Server, ServiceExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = Router::new();
    let routes_hello = router.route(
        "/hello1",
        axum::routing::get(|| async { Html("Hello, World!") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {}", addr);
    Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

struct HelloParams {
    name: Option<String>,
}

async fn handler_hello() -> impl IntoResponse {
    Html("Hello <strong>World</strong>!!!")
}
