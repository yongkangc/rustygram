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
