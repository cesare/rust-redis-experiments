use futures_util::stream::StreamExt;

mod redis_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = redis_client::create_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub.subscribe("channel-a").await?;

    let mut stream = pubsub.on_message();
    while let Some(message) = stream.next().await {
        println!("Receive message: {:?}", message);
    }

    Ok(())
}
