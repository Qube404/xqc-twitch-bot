use std::env;
use dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready) {
        println!("Connected");
        let user = ctx
            .http
            .get_user(266059446159933452)
            .await
            .expect("Expected a valid user");

        user
            .direct_message(&ctx, |m| m.content("Test"))
            .await
            .expect("Expected message to send");
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
