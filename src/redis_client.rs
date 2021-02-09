use redis::{Client, RedisError};
use redis::aio::Connection;

pub async fn create_connection() -> Result<Connection, RedisError> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_async_connection().await
}
