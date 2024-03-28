// This is an example file of how rustygram can be integrated easily into your trading systems. 
// This example uses execution on Binance
// In `.env` - `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID` has to be declared first. 

use std::env;

use chrono::DateTime;
use chrono_tz::Tz;
use dotenv::dotenv;
use rustygram::bot::Bot;

pub struct TradeSuccessNotification {
    pub listing_id: u32,
    pub ticker: String,
    pub market_buy_qty: f64,
    pub market_buy_price: f64,
    pub stop_loss_price: f64,
    pub take_profit_price: f64,
    pub timestamp: DateTime<Tz>,
}

impl TradeSuccessNotification {
    fn craft_message(&self) -> String {
        format!(
            "listing ID: {}\nTicker: {}\nMarket Buy at {}qty at ${}\nStop loss: {}\nTake profit: {}\n{}",
            self.listing_id,
            self.ticker,
            self.market_buy_qty,
            self.market_buy_price,
            self.stop_loss_price,
            self.take_profit_price,
            self.timestamp
        )
    }
}

pub struct TradeFailureNotification {
    pub listing_id: u32,
    pub ticker: String,
    pub timestamp: DateTime<Tz>,
    pub error: String,
}

impl TradeFailureNotification {
    fn craft_message(&self) -> String {
        format!(
            "Listing ID: {}\nExecution failed for {} due to Error: {}\n{}",
            self.listing_id, self.ticker, self.error, self.timestamp
        )
    }
}

pub async fn send_success_notification(bot: &Bot, message: &TradeSuccessNotification) {
    let message = message.craft_message();
    let _ = bot.send_message(&message, None).await;
}

pub async fn send_failure_notification(bot: &Bot, message: &TradeFailureNotification) {
    let message = message.craft_message();
    let _ = bot.send_message(&message, None).await;
}

pub async fn send_network_client_failure_notification(bot: &Bot, error: &str) {
    let message = format!("Network client failed with error: {}", error);
    let _ = bot.send_message(&message, None).await;
}

/// Create a new bot using the Telegram bot token and chat ID from the environment variables.
///
/// # Example
///
/// ```no_run
/// use notifications::notifications::create_bot;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Assuming the environment variables are set
/// let bot = create_bot();
/// # Ok(())
/// # }
/// ```
pub fn create_bot() -> Bot {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or_else(|_| "".to_string());
    let chat_id = env::var("TELEGRAM_CHAT_ID").unwrap_or_else(|_| "".to_string());
    Bot::new(token, chat_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_success_notification_craft_message() {
        let time = chrono::DateTime::parse_from_rfc3339("2020-12-31T23:59:59.999999999Z").unwrap();
        let singapore_time = time.with_timezone(&chrono_tz::Singapore);
        let notification = TradeSuccessNotification {
            listing_id: 1,
            ticker: "BTC".to_string(),
            market_buy_qty: 0.1,
            market_buy_price: 10000.0,
            stop_loss_price: 9000.0,
            take_profit_price: 11000.0,
            timestamp: singapore_time,
        };

        let message = notification.craft_message();
        assert_eq!(
            message,
            "listing ID: 1\nTicker: BTC\nMarket Buy at 0.1qty at $10000\nStop loss: 9000\nTake profit: 11000\n2021-01-01 07:59:59.999999999 +08"
        );
    }

    #[test]
    fn test_trade_failure_notification_craft_message() {
        let time = chrono::DateTime::parse_from_rfc3339("2020-12-31T23:59:59.999999999Z").unwrap();
        let singapore_time = time.with_timezone(&chrono_tz::Singapore);

        let notification = TradeFailureNotification {
            listing_id: 1,
            ticker: "BTC".to_string(),
            timestamp: singapore_time,
            error: "Insufficient funds".to_string(),
        };

        let message = notification.craft_message();
        assert_eq!(
            message,
            "listing ID: 1\nExecution failed for BTC due to Error: Insufficient funds\n2021-01-01 07:59:59.999999999 +08"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_send_success_notification() {
        let bot = create_bot();
        let time = chrono::DateTime::parse_from_rfc3339("2020-12-31T23:59:59.999999999Z").unwrap();
        let singapore_time = time.with_timezone(&chrono_tz::Singapore);
        let notification = TradeSuccessNotification {
            listing_id: 1,
            ticker: "BTC".to_string(),
            market_buy_qty: 0.1,
            market_buy_price: 10000.0,
            stop_loss_price: 9000.0,
            take_profit_price: 11000.0,
            timestamp: singapore_time,
        };

        send_success_notification(&bot, &notification).await;
    }
}
