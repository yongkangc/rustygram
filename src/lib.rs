/// High Level Wrapper API of Bot.
///
/// This module provides a high-level API for interacting with Telegram's bot API. It provides
/// functionality for creating a bot with a token and chat ID, and for sending messages.
///
/// The `create_bot` function is used to create a new `Bot` instance. The `bot_token` parameter
/// should be the token provided by the BotFather on Telegram, and `chat_id` should be the ID
/// of the chat where the bot should send messages.
///
/// The `send_message` function is used to send a message to the chat associated with the `Bot`.
/// The `msg` parameter is the text of the message to send, and the `options` parameter can be
/// used to specify additional options like parse mode.
use bot::Bot;
pub mod bot;
pub mod errors;
pub mod tests;
pub mod types;
pub mod utils;

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
