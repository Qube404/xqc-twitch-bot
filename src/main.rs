use std::env;
use dotenv;

use serenity::prelude::{GatewayIntents, Client};

// Hard coded my user ID as I don't intend this bot to message anyone else.
const QUBE_ID: u64 = 266059446159933452;

mod discord_connection;
mod twitch_connection;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::DIRECT_MESSAGES;

    let mut client =
        Client::builder(&token, intents)
        .event_handler(discord_connection::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
