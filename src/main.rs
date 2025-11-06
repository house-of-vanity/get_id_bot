use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting get_id_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(user) = msg.from() {
            let user_id = user.id;
            let username = user.username.as_ref().map(|u| u.as_str()).unwrap_or("no_username");
            let first_name = &user.first_name;

            log::info!(
                "Request from user: {} (@{}) ID: {}",
                first_name, username, user_id
            );

            bot.send_message(msg.chat.id, format!("`{}`", user_id))
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
        }
        Ok(())
    })
    .await;
}
