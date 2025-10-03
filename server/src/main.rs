use dotenvy::{dotenv, var};
use axum::{Error, Router};

mod db;
mod models;
mod schema;
mod routers;
mod controllers;

use routers::user_routes::user_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().unwrap();

    let app = Router::new()
        .nest("/api/users", user_router());

    let address = format!("{}:{}", var("HOST").unwrap(), var("PORT").unwrap());

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Listening @ {}", &address);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
