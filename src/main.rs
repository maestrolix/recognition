use routes::craete_app;

pub mod db_connection;
pub mod errors;
pub mod middleware;
pub mod ml;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = craete_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
