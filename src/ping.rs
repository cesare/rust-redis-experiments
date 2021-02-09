use redis::RedisError;

mod redis_client;

async fn ping_request() -> Result<String, RedisError> {
    let mut connection = redis_client::create_connection().await?;
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
