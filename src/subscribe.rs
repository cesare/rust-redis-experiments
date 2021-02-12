use futures_util::stream::StreamExt;
use redis::Msg;
use serde::Deserialize;
use serde_json::Value;
use structopt::StructOpt;

use std::convert::TryFrom;

mod redis_client;

#[derive(StructOpt)]
#[structopt(name = "subscribe")]
struct SubscriptionConfig {
    #[structopt(short = "c", long = "channel", default_value = "channel-a")]
    channel: String,
}

#[derive(Deserialize, Debug)]
struct TextMessage {
    text: String,
}

#[derive(Debug)]
enum Message {
    Text(TextMessage),
    Unknown(Value),
}

impl TryFrom<Value> for Message {
    type Error = serde_json::Error;

    fn try_from(json: Value) -> Result<Self, Self::Error> {
        if let Some(message_type) = json.get("message_type").and_then(|v| v.as_str()) {
            let message = match message_type {
                "text" => Message::Text(serde_json::from_value(json)?),
                _ => Message::Unknown(json)
            };
            Ok(message)
        } else {
            Ok(Message::Unknown(json))
        }
    }
}

async fn show_message(message: &Msg) -> Result<(), Box<dyn std::error::Error>> {
    let payload: String = message.get_payload()?;
    if let Ok(json) = serde_json::from_str::<Value>(&payload) {
        if let Ok(msg) = Message::try_from(json) {
            println!("{:?}", msg);
        }
    } else {
        println!("Received non-JSON message: {}", payload);
    }

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
