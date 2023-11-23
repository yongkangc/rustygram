/// Create a new bot using the Telegram bot token and chat ID from the environment variables.
///
/// # Example
///
/// ```
/// use rustygram::bot::Bot;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Assuming the environment variables are set
/// let bot = create_bot()?;
/// // Use the bot...
/// # Ok(())
/// # }
/// ```
fn create_bot() -> Bot {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");
    Bot::new(token, chat_id)
}

#[tokio::main]
/// Send a message to the chat associated with the bot.
fn send_message() -> Result<(), ErrorResult> {
    let bot = create_bot();
    let msg = "Hello, world!";
    let options = None;
    bot.send_message(msg, options)
}
