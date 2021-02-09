use redis::{Client, RedisError};
use redis::aio::Connection;

async fn create_redis_connection() -> Result<Connection, RedisError> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_async_connection().await
}

async fn ping_request() -> Result<String, RedisError> {
    let mut connection = create_redis_connection().await?;
    redis::cmd("PING").arg("hello")
        .query_async(&mut connection)
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = ping_request().await?;
    println!("{}", response);
    Ok(())
}
