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
    /// `BOT_TOKEN` - telegram bot's token
    /// `CHAT_ID` - telegram bot's chat id
    ///
    /// These environment variable names are used for testing purpose only.
    fn get_keys() -> Result<(String, String), (bool, bool)> {
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        let mut results = (false, false);

        if let Ok(st) = env::var("BOT_TOKEN") {
            keys.0 = st;
            results.0 = true;
        }

        if let Ok(st) = env::var("CHAT_ID") {
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
            return "BOT_TOKEN and CHAT_ID environment variable are not set";
        }
        if !errors.0 {
            return "BOT_TOKEN environment variable is not set";
        }
        return "CHAT_ID environment variable is not set";
    }

    fn get_instance() -> Bot {
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        match get_keys() {
            Ok(ret_keys) => keys = ret_keys,
            Err(errs) => assert!(false, "{}", get_ret_error_msg(errs)),
        }

        create_bot(&keys.0.to_owned(), &keys.1.to_owned())
    }

    // #[test]
    // fn test_send_message_simple() {
    //     let instance = get_instance();

    //     if let Err(_) = send_message(&instance, "test", None) {
    //         assert!(false);
    //     }
    // }
}
