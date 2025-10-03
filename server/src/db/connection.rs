use diesel::prelude::*;
use dotenvy::dotenv;
use std::env::var;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = var("DATABASE_URL").expect("Database URL mnust be set!");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
