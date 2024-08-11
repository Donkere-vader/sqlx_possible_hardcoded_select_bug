use std::env;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, query};

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await
        .unwrap();

    let record = query!("SELECT true as value")
        .fetch_one(&pool)
        .await
        .unwrap();

    // I would expect just `true` but it is `Some(true)`
    println!("{:?}", record.value);
}
