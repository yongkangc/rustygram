use std::fmt::{Display, Error, Formatter};

use reqwest::{multipart, StatusCode};
/// ErrorResult usually returned to indicate result from calling APIs related
/// functions.
#[derive(Debug)]
pub struct ErrorResult {
    pub code: u16,   // error returned code
    pub msg: String, // error string description
}

/// Telegram's error result.
/// In case of error occurred as part of telegram API calling, then this struct
/// will be formed and returned.
#[derive(Debug, serde::Deserialize)]
pub struct TelegramErrorResult {
    pub ok: bool,
    pub error_code: i32,
    pub description: String,
}

impl Display for ErrorResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.msg)
    }
}
impl From<std::io::Error> for ErrorResult {
    fn from(err: std::io::Error) -> Self {
        ErrorResult {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            msg: format!("IO Error: {}", err),
        }
    }
}

impl From<reqwest::Error> for ErrorResult {
    fn from(err: reqwest::Error) -> Self {
        ErrorResult {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            msg: format!("HTTP Request Error: {}", err),
        }
    }
}
