use redis::{Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_async_connection().await?;
    let response: String =
        redis::cmd("PING")
            .arg("hello")
            .query_async(&mut connection)
            .await?;
    println!("{}", response);
    Ok(())
}
