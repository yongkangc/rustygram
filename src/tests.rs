#[cfg(test)]
mod test {
    use crate::{
        types::{SendMessageOption, SendMessageParseMode},
        *,
    }; // import lib.rs
    use std::env;

    /// Reading bot token, and chat id for inteacting with telegram bot.
    ///
    /// Set environment variable as follows before running test
    /// `TEST_BOT_TOKEN` - telegram bot's token
    /// `TEST_CHAT_ID` - telegram bot's chat id
    ///
    /// These environment variable names are used for testing purpose only.
    fn get_keys() -> Result<(String, String), String> {
        dotenv::dotenv().ok();

        let bot_token = env::var("TEST_BOT_TOKEN")
            .map_err(|_| "TEST_BOT_TOKEN environment variable is not set")?;
        let chat_id =
            env::var("TEST_CHAT_ID").map_err(|_| "TEST_CHAT_ID environment variable is not set")?;

        Ok((bot_token, chat_id))
    }

    /// Create test bot instance with given bot token, and chat id.
    fn get_bot() -> Bot {
        let keys = get_keys().expect("Failed to read environment variables");
        create_bot(&keys.0, &keys.1)
    }

    #[tokio::test]
    async fn test_send_message_simple() {
        let bot = get_bot();

        let res = bot.send_message("test_send_message_simple", None).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_send_markdown_message() {
        let bot = get_bot();
        let m1 = bot.send_message(
            r#"\[rustygram\] __MarkdownV2__  
                *async msg 1*  
                `Tap to copy this text`\.  
                You can visit my [website](https://github.com/extremelySunnyYK)\.  
                Woot\!"#,
            Some(SendMessageOption {
                parse_mode: Some(SendMessageParseMode::MarkdownV2),
            }),
        );

        let m2 = bot.send_message(
            r#"\[rustygram\] __MarkdownV2__  
                *async msg 2*  
                `Tap to copy this text`\.  
                You can visit my [website](https://github.com/extremelySunnyYK)\.  
                Woot\!"#,
            Some(SendMessageOption {
                parse_mode: Some(SendMessageParseMode::MarkdownV2),
            }),
        );

        assert!(m1.await.is_ok());
        assert!(m2.await.is_ok());
    }

    #[tokio::test]
    async fn test_send_html_message() {
        let bot = get_bot();
        let m1 = bot.send_message(
            r#"[rustygram] <u>HTML style</u> - <b>async msg 1</b>
                <code>Tap to copy this text</code>.
                You can visit my <a href="https://github.com/ExtremelySunnyYK">website</a>.
                Woot!"#,
            Some(SendMessageOption {
                parse_mode: Some(SendMessageParseMode::HTML),
            }),
        );
        let m2 = bot.send_message(
            r#"[rustygram] <u>HTML style</u> - <b>async msg 2</b>
                    <code>Tap to copy this text</code>.
                    You can visit my <a href="https://github.com/ExtremelySunnyYK">website</a>.
                    Woot!"#,
            Some(SendMessageOption {
                parse_mode: Some(SendMessageParseMode::HTML),
            }),
        );

        assert!(m1.await.is_ok());
        assert!(m2.await.is_ok());
    }
}
