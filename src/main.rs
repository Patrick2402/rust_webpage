use axum::{routing::get, Router};
use hyper::Server;
use site::{test_page, handle_assets};
mod site;

async fn test() -> &'static str {
    "Hello World"
}

async fn user() -> &'static str {
    "User"
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Just trying to compile some good rust and earn money!" }),
        )
        .route("/_assets/*path",get(handle_assets))
        .route("/test", get(test_page))
        .route("/users", get(user));

    // run it with hyper on localhost:3000
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[derive(Debug)]
// struct Foo {
//     number: u32
// }

// fn main() {
//     let foo = Foo { number: 10};
//     dbg!(foo);
// }