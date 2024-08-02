use dotenv::dotenv;
use server::run;
// use sqlx::types::chrono::{DateTime, Local, NaiveDateTime, Utc};
// use sqlx::{postgres::PgPoolOptions, Pool};
// use std::result;
// use std::time::SystemTime;
// use std::{env, str::FromStr};

// use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&env::var("DATABASE_URL").unwrap())
    //     .await
    //     .expect("Failed to connect to Postgres");

    if let Err(e) = run().await {
        eprintln!("{}", e);
    }
}
