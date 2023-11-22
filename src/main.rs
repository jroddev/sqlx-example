use futures::TryStreamExt;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Row,
};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
struct TestEntry {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let record: PgRow = sqlx::query(
        r#"
            INSERT INTO test ( value )
            VALUES ( $1 )
            RETURNING id
        "#,
    )
    .bind(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>(),
    )
    .fetch_one(&pool)
    .await?;

    let inserted_id: Uuid = record.get("id");
    println!("Inserted ID: {inserted_id:?}");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test")
        .fetch_one(&pool)
        .await?;

    println!("Count: {count}");

    let mut stream = sqlx::query_as::<_, TestEntry>("SELECT * FROM test").fetch(&pool);
    while let Some(row) = stream.try_next().await? {
        println!("Entry: {row:?}");
    }

    Ok(())
}
