use std::fmt::{Display, Error, Formatter};
/// ErrorResult usually returned to indicate result from calling APIs related
/// functions.
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
