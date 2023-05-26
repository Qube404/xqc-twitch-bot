use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;

pub struct Handler;

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
            .get_user(super::QUBE_ID)
            .await
            .expect("Expected a valid user");

        user
            .direct_message(&ctx, |m| m.content("Test"))
            .await
            .expect("Expected message to send");
    }
}
