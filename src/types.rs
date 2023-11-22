/// Parse mode for `sendMessage` API
pub enum SendMessageParseMode {
    /// MarkdownV2 style
    MarkdownV2,

    /// HTML style
    HTML,
}

/// Options which can be used with `sendMessage` API
pub struct SendMessageOption {
    /// Parse mode
    pub parse_mode: Option<SendMessageParseMode>,
}

/// Status code indicating the result of APIs related function call.
pub enum StatusCode {
    /// Success
    Success = 0,

    /// Internal error due to various internal operations.
    /// Whenever Telegram's related operations occurred with error, then this
    /// value will be used.
    ErrorInternalError,
}

/// Request Object for `sendMessage` API
/// See https://core.telegram.org/bots/api#sendmessage
/// NOTE: serde::Serialize can work with &str
#[derive(Debug, serde::Serialize)]
pub struct RequestObj {
    chat_id: String,
    text: String,
    // this is required unfortunately, see https://github.com/serde-rs/serde/issues/947
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
}

impl RequestObj {
    /// Create a new `RequestObj` with given chat id and message.
    pub fn new(chat_id: &str, text: &str, parse_mode: Option<String>) -> Self {
        Self {
            chat_id: chat_id.to_owned(),
            text: text.to_owned(),
            parse_mode,
        }
    }
}
