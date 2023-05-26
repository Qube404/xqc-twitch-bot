use std::sync::mpsc::Sender;

use tokio;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

// Reads xqc's messages from his twitch chat.
pub async fn read_xqc_messages(tx: Sender<String>) {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::task::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(message) => {
                    if message.message_text == "test" {
                        tx.send(message.message_text).unwrap();
                    }
                }

                _ => ()
            }
        }
    });

    client
        .join("xqc".to_owned())
        .unwrap();

    join_handle
        .await
        .unwrap();
}
