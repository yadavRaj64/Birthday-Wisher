use std::env;

use dotenvy::dotenv;
use sqlx::{Error, PgPool, Pool, Postgres};

pub async fn establish_connect() -> Result<Pool<Postgres>, Error> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("Please set DATABASE_URL in your environment");

    let pool = PgPool::connect(&database_url).await;
    pool
}
