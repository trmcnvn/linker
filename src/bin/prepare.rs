use std::env;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let (mut client, connection) =
        tokio_postgres::connect(&database_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    Ok(())
}
