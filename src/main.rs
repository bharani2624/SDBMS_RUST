use std::sync::Arc;
use axum::{Router, routing::{get, post, delete}};
use tower_http::cors::{Any, CorsLayer};
use db::connection::connect_db;

mod db;
mod routes;
mod model;

use routes::student_routes::{add, getall, get_student, del_student};

#[tokio::main]
async fn main() {
    let db = connect_db().await;
    let db = Arc::new(db);

    let app = Router::new()
        .route("/students", post(add).get(getall))
        .route("/students/{id}", get(get_student).delete(del_student))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Kolaru in connection");
    println!("Server running at http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}
