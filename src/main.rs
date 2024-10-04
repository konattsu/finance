use std::env;

use finance::{YahooError, YahooFinanceInfo, YahooFinanceResp};
use google_chat_webhook::{Message, Reply};

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let webhook_url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
    let thread_id =
        env::var("WEBHOOK_THREAD_REPLY").expect("WEBHOOK_THREAD_REPLY must be set");

    let message_text = match process_yahoo_finance() {
        Ok(val) => val,
        Err(e) => format!("YahooFinanceError - {}", e),
    };

    if let Err(e) = google_chat_webhook::blocking::post_google_chat_webhook(
        &webhook_url,
        Message::new(message_text, Some(Reply::new(thread_id, true))),
    ) {
        eprintln!("GoogleChatWebhookError - {}", e);
        return Err(e.into());
    }

    Ok(())
}

fn process_yahoo_finance() -> Result<String, YahooError> {
    let yahoo_finance_url =
        env::var("YAHOO_FINANCE_URL").expect("YAHOO_FINANCE_URL must be set");

    let resp = YahooFinanceResp::fetch(&yahoo_finance_url)?;
    let info = YahooFinanceInfo::new(&resp)?;

    if info.updated_today_in_ja() {
        Ok(info.get_result_that_day_ja())
    } else {
        Ok(format!("{}\n本日はお休み", info.get_name()))
    }
}
