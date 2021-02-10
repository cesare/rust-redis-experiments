use redis::{AsyncCommands, RedisError};

mod redis_client;

async fn publish_request() -> Result<(), RedisError> {
    let mut connection = redis_client::create_connection().await?;
    connection.publish("channel-a", "test for echo").await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    publish_request().await?;
    Ok(())
}
