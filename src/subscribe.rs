use futures_util::stream::StreamExt;
use redis::Msg;
use structopt::StructOpt;

mod redis_client;

#[derive(StructOpt)]
#[structopt(name = "subscribe")]
struct SubscriptionConfig {
    #[structopt(short = "c", long = "channel", default_value = "channel-a")]
    channel: String,
}

async fn show_message(message: &Msg) -> Result<(), Box<dyn std::error::Error>> {
    let payload: String = message.get_payload()?;
    println!("Received message: {}", payload);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SubscriptionConfig::from_args();

    let connection = redis_client::create_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub.subscribe(&config.channel).await?;

    let mut stream = pubsub.on_message();
    while let Some(message) = stream.next().await {
        show_message(&message).await?;
    }

    Ok(())
}
