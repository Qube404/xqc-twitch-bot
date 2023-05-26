use std::env;
use dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data: Ready) {
        println!("Connected");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::DIRECT_MESSAGES;

    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
