use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::prelude::{GatewayIntents, Client};

// Hard coded my user ID as I don't intend this bot to message anyone else.
const QUBE_ID: u64 = 266059446159933452;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Using the ready event as it is triggered as soon as the bot connects.
    // The bot should always be ready to send a message from the twitch api
    // so it starts as soon as the bot is connected as opposed to waiting 
    // for some discord reliant event.
    async fn ready(&self, ctx: Context, _data: Ready) {
        println!("Connected");
        let user = ctx
            .http
            .get_user(QUBE_ID)
            .await
            .expect("Expected a valid user");

        user
            .direct_message(&ctx, |m| m.content("Test"))
            .await
            .expect("Expected message to send");
    }
}

pub async fn write_xqc_messages() {
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
