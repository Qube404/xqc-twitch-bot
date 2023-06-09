use std::env;
use std::sync::mpsc::Receiver;

use serenity::prelude::{GatewayIntents, Client};

// Hard coded my user ID as I don't intend this bot to message anyone else.
const QUBE_ID: u64 = 266059446159933452;

pub async fn write_xqc_messages(rx: Receiver<String>) {
    println!("Succesfully inside discord thread.");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::DIRECT_MESSAGES;

    let mut client =
        Client::builder(&token, intents)
        .await
        .expect("Err creating client");

    let user = client
        .cache_and_http
        .http
        .get_user(QUBE_ID)
        .await
        .expect("Expected a valid user");

    let cache_http = &client
        .cache_and_http;

    for receiver in rx {
        if receiver == "ping test" {
            println!("Succesfully received ping!");
        } else {
            user.direct_message(cache_http, |m| m.content(receiver)).await.unwrap();
        }
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
