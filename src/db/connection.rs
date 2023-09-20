use sqlx::PgPool;
use std::env;

pub async fn connect() -> sqlx::Result<PgPool> {
    let database_url = "postgres://postgres:secret@localhost:5432/address_book";
    // let database_url = env::var("DATABASE_URL")
    //     .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
