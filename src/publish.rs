use redis::{AsyncCommands, RedisError};
use structopt::StructOpt;

mod redis_client;

#[derive(StructOpt)]
#[structopt(name = "publish")]
struct PublishConfig {
    #[structopt(short = "c", long = "channel", default_value = "channel-a")]
    channel: String,

    #[structopt(name = "message")]
    message: String,
}

async fn publish_request(config: &PublishConfig) -> Result<(), RedisError> {
    let mut connection = redis_client::create_connection().await?;
    connection.publish(&config.channel, &config.message).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PublishConfig::from_args();
    publish_request(&config).await?;
    Ok(())
}
