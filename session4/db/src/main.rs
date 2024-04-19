use futures::TryStreamExt;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("start program");

    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // if you want to see span set export RUST_LOG=debug
    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    // let pool = sqlx::sqlite::SqlitePoolOptions::new()
    //     .connect(&db_url)
    //     .await?;

    //  Run Migrations

    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("migration done");

    // NOTE: in case not struct binding
    // let messages = sqlx::query("SELECT id, message FROM messages")
    //     .map(|row: sqlx::sqlite::SqliteRow| {
    //         let id: i64 = row.get(0);
    //         let message: String = row.get(1);
    //         (id, message)
    //     })
    //     .fetch_all(&pool)
    //     .await?;

    // Print the messages
    // for (id, message) in messages {
    //     println!("{id}: {message}");
    // }

    // NOTE: incase fetch stream
    println!("--- stream ---");

    let messages: Vec<Message> = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(&pool)
        .await?;
    println!("{messages:?}");

    update_message(4, "Updated Message", &pool).await?;

    let mut message_stream =
        sqlx::query_as::<_, Message>("SELECT id , message FROM messages").fetch(&pool);

    while let Some(message) = message_stream.try_next().await? {
        println!("{message:?}");
    }

    Ok(())
}
