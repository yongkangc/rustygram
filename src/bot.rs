use reqwest::Client;
use std::{sync::Arc, time::Duration};

use crate::{
    errors::{ErrorResult, TelegramErrorResult},
    types::{RequestObj, SendMessageOption, StatusCode},
    utils,
};

pub const TELEGRAM_API_URL: &str = "https://api.telegram.org";
const SEND_MESSAGE_METHOD: &str = "sendMessage";

/// A requests sender.
///
/// This is the main type of the library, it allows to send requests to the
/// [Telegram Bot API] and download files.
///
/// ## Clone cost
///
/// `Bot::clone` is relatively cheap, so if you need to share `Bot`, it's
/// recommended to clone it, instead of wrapping it in [`Arc<_>`].
///
/// [`Arc`]: std::sync::Arc
/// [Telegram Bot API]: https://core.telegram.org/bots/api
#[must_use]
#[derive(Debug, Clone)]
pub struct Bot {
    pub token: Arc<str>,
    pub chat_id: Arc<str>,
    pub api_url: Arc<reqwest::Url>,
    pub client: Client,
}

/// Constructors
impl Bot {
    /// Creates a new `Bot` with the specified token and the default
    /// [http-client](reqwest::Client).
    ///
    /// # Panics
    /// If it cannot create [`reqwest::Client`].
    ///
    pub fn new<S>(token: S, chat_id: S) -> Self
    where
        S: Into<String>,
    {
        let client = default_reqwest_settings()
            .build()
            .expect("Client creation failed");

        Self::with_client(token, chat_id, client)
    }

    /// Creates a new `Bot` with the specified token and your
    /// [`reqwest::Client`].
    ///
    /// # Caution
    ///
    /// Your custom client might not be configured correctly to be able to work
    /// in long time durations, see [issue 223].
    ///
    /// [`reqwest::Client`]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
    /// [issue 223]: https://github.com/teloxide/teloxide/issues/223
    pub fn with_client<S>(token: S, chat_id: S, client: Client) -> Self
    where
        S: Into<String>,
    {
        let token = Into::<String>::into(token).into();
        let chat_id = Into::<String>::into(chat_id).into();
        let api_url = Arc::new(
            reqwest::Url::parse(TELEGRAM_API_URL)
                .expect("Failed to parse default Telegram bot API url"),
        );

        Self {
            token,
            chat_id,
            api_url,
            client,
        }
    }
}

/// Core Functionality
impl Bot {
    /// Sends a request to the Telegram Bot API asynchronously
    pub async fn send_message(
        &self,
        msg: &str,
        options: Option<SendMessageOption>,
    ) -> Result<(), ErrorResult> {
        let request_json_obj = self.build_request_obj(msg, options);

        let response = self
            .client
            .post(method_url(
                self.api_url(),
                self.token(),
                SEND_MESSAGE_METHOD,
            ))
            .json(&request_json_obj)
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    match res.json::<TelegramErrorResult>().await {
                        Ok(err_result) => {
                            return utils::create_error_result_str(
                                StatusCode::ErrorInternalError,
                                &err_result.description.to_owned(),
                            )
                        }
                        Err(_) => {
                            return utils::create_error_result_str(
                                StatusCode::ErrorInternalError,
                                "Error converting telegram error response to json",
                            )
                        }
                    }
                }
            }
            Err(e) => {
                return utils::create_error_result_str(
                    StatusCode::ErrorInternalError,
                    &format!("Error sending HTTP request; err={}", e),
                )
            }
        }
    }
    fn build_request_obj(&self, msg: &str, options: Option<SendMessageOption>) -> RequestObj {
        let parse_mode = options
            .as_ref()
            .and_then(|option| option.parse_mode.as_ref()) // Avoid repeated unwrap calls
            .map(|mode| utils::get_send_message_parse_mode_str(mode).to_owned());

        RequestObj::new(&self.chat_id, msg, parse_mode)
    }
}

/// Getters
impl Bot {
    /// Returns currently used token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns currently used http-client.
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns currently used token API url.
    #[must_use]
    pub fn api_url(&self) -> reqwest::Url {
        reqwest::Url::clone(&*self.api_url)
    }
}

/// ********************** Utilities **********************

/// Returns a reqwest client builder with default settings.
///
/// Client built from default settings is supposed to work over long time
/// durations, see the [issue 223].
///
/// The current settings are:
///  - A connection timeout of 5 seconds.
///  - A timeout of 17 seconds.
///  - `tcp_nodelay` is on.
///
/// ## Notes
///
/// 1. The settings may change in the future.
/// 2. If you are using the polling mechanism to get updates, the timeout
///    configured in the client should be bigger than the polling timeout.
/// 3. If you alter the current settings listed above, your bot will not be
///    guaranteed to work over long time durations.
///
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
fn default_reqwest_settings() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(17))
        .tcp_nodelay(true)
}

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(base: reqwest::Url, token: &str, method_name: &str) -> reqwest::Url {
    base.join(&format!("/bot{token}/{method_name}"))
        .expect("failed to format url")
}

#[cfg(test)]
mod tests {
    use crate::bot::{method_url, SEND_MESSAGE_METHOD, TELEGRAM_API_URL};

    #[test]
    fn method_url_test() {
        let url = method_url(
            reqwest::Url::parse(TELEGRAM_API_URL).unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            SEND_MESSAGE_METHOD,
        );

        assert_eq!(
            url.as_str(),
            format!(
                "https://api.telegram.org/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/{}",
                SEND_MESSAGE_METHOD
            )
        );
    }
}
