use dotenv;

mod discord_connection;
mod twitch_connection;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok();

    twitch_connection::read_xqc_messages()
        .await;

    discord_connection::write_xqc_messages()
        .await;
}
