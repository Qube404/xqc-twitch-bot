use std::sync::mpsc::Sender;

use tokio;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

// Reads xqc's messages from his twitch chat.
pub async fn read_xqc_messages(tx: Sender<String>) {
    println!("Succesfully inside twitch thread.");

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::task::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(message) = message {
                if message.sender.login.as_str() == "qube_404" {
                    tx.send(message.message_text).expect("Could not send message");
                }
            }
        }
    });

    client
        .join("xqc".to_owned())
        .expect("Could not join channel");

    join_handle
        .await
        .expect("Join handle failed.");
}
