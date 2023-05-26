use dotenv;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

mod discord_connection;
mod twitch_connection;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok();

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    twitch_connection::read_xqc_messages(tx)
        .await;

    discord_connection::write_xqc_messages(rx)
        .await;
}
