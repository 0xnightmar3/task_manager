use axum::{Error, Router};

mod routers;

use routers::user_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new()
        .nest("/api/users", user_router());

    let listener = tokio::net::TcpListener::bind("localhost:1338").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
