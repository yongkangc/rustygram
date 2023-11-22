use bot::Bot;

pub mod bot;
pub mod errors;
pub mod tests;
pub mod types;
pub mod utils;

/// High Level Wrapper API of Bot.

/// Create a Bot to interact with APIs.
/// Returns a `Bot` configured with the provided bot token and chat id.
///
/// # Arguments
/// * `bot_token` - a string of bot token
/// * `chat_id` - a chat id string to send message
pub fn create_bot(bot_token: &str, chat_id: &str) -> Bot {
    Bot::new(bot_token, chat_id)
}

/// Send message asynchronously.
/// Return `Result<(), ErrorResult>`.
///
/// # Arguments
///
/// * `bot` - `Bot` to send message to telegram's chat
/// * `msg` - message to send
/// * `options` - options for sending message
pub async fn send_message(
    bot: &Bot,
    msg: &str,
    options: Option<types::SendMessageOption>,
) -> Result<(), errors::ErrorResult> {
    bot.send_message(msg, options).await
}
