use {env_logger::Env, log::info, teloxide::prelude::*};

//  TELOXIDE_TOKEN=6268131418:AAHcotYvZ3FI0lrJHmyMWUSOseyY4dxzM0I cargo run --bin bot

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("bot started");
    let bot = Bot::from_env();
    bot.send_message(ChatId(1712332550), "Hello telegram world!")
        .send()
        .await
        .unwrap();
}
