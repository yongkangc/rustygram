#[cfg(test)]
mod test {
    use crate::*; // import lib.rs
    use std::env;

    /// Reading bot token, and chat id for inteacting with telegram bot.
    /// Return tuple of (bot token, and telegrama's chat id) if successfully read,
    /// otherwise return a tuple of success flag. Both returned tuples are returned
    /// in order of bot token, and chat id.
    ///
    /// Set environment variable as follows before running test
    /// `TEST_BOT_TOKEN` - telegram bot's token
    /// `TEST_CHAT_ID` - telegram bot's chat id
    ///
    /// These environment variable names are used for testing purpose only.
    fn get_keys() -> Result<(String, String), (bool, bool)> {
        dotenv::dotenv().ok();
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        let mut results = (false, false);

        if let Ok(st) = env::var("TEST_BOT_TOKEN") {
            keys.0 = st;
            results.0 = true;
        }

        if let Ok(st) = env::var("TEST_CHAT_ID") {
            keys.1 = st;
            results.1 = true;
        }

        if results.0 && results.1 {
            Ok(keys)
        } else {
            Err(results)
        }
    }

    /// Get a proper error message from the tuple of success flag
    /// for getting value from environment variables.
    /// See `get_keys()`.
    fn get_ret_error_msg(errors: (bool, bool)) -> &'static str {
        if !errors.0 && !errors.1 {
            return "TEST_BOT_TOKEN and TEST_CHAT_ID environment variable are not set";
        }
        if !errors.0 {
            return "TEST_BOT_TOKEN environment variable is not set";
        }
        return "TEST_CHAT_ID environment variable is not set";
    }

    fn get_instance() -> Bot {
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        match get_keys() {
            Ok(ret_keys) => keys = ret_keys,
            Err(errs) => assert!(false, "{}", get_ret_error_msg(errs)),
        }

        create_bot(&keys.0.to_owned(), &keys.1.to_owned())
    }

    #[tokio::test]
    async fn test_send_message_simple() {
        let bot = get_instance();

        let res = bot.send_message("test_send_message_simple", None).await;
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
