use crate::{
    errors::ErrorResult,
    types::{SendMessageParseMode, StatusCode},
};

/// Get a string representing of specified parse mode.
///
/// This function returns static string as it is expected to be used across
/// the lifetime of the application. So it has no need to return new instance
/// of `String` every time.
///
/// # Arguments
/// * `mode` - parse mode
pub fn get_send_message_parse_mode_str(mode: SendMessageParseMode) -> &'static str {
    match mode {
        SendMessageParseMode::MarkdownV2 => "MarkdownV2",
        SendMessageParseMode::HTML => "HTML",
    }
}

/// Create an `ErrorResult` from input of string slice.
///
/// # Arguments
///
/// * `code` - `StatusCode`
/// * `msg` - message string to add as an error description
pub fn create_error_result_str(code: StatusCode, msg: &str) -> Result<(), ErrorResult> {
    Err(ErrorResult {
        code: code as u16,
        msg: msg.to_string(),
    })
}
