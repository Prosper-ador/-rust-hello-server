use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/*use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define a router with a simple GET "/" handler
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Use Hyper's Server to bind and serve Axum's router
    let listener = tokio::net::TcpListener::bind(&addr)
        //.serve(app.into_make_service())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}*/

