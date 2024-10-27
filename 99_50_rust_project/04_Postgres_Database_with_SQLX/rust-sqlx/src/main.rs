use sqlx::{postgres::PgConnectOptions, Pool, Postgres, Result};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let url = std::env::var("DATABASE_URL").unwrap();
    dbg!(url);
    // 1) Create a connection pool
    let pool_options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("1234");

    let pool = Pool::<Postgres>::connect_with(pool_options).await?;

    // 2) Create table if not exist yet
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS ticket (
        id bigserial,
        name text
    );"#,
    )
    .execute(&pool)
    .await?;

    // 3) Insert a new ticket

    // 4) Select all tickets

    // 5) Select query with map() (build the Ticket manually)

    // 6) Select query_as (using derive FromRow)
    Ok(())
}
