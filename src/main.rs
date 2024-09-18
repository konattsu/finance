use std::env;

use finance::{YahooError, YahooFinanceInfo, YahooFinanceResp};
use google_chat_webhook::Message;

fn main() -> Result<(), anyhow::Error> {
    env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();
    let webhook_url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
    let thread_id =
        env::var("WEBHOOK_THREAD_REPLY").expect("WEBHOOK_THREAD_REPLY must be set");

    let message_text = process_yahoo_finance()?;

    google_chat_webhook::post_google_chat_webhook(
        &webhook_url,
        Message::new(message_text, Some(thread_id.clone())),
    )?;

    Ok(())
}

fn process_yahoo_finance() -> Result<String, YahooError> {
    let yahoo_finance_url =
        env::var("YAHOO_FINANCE_URL").expect("YAHOO_FINANCE_URL must be set");

    let resp = YahooFinanceResp::fetch(&yahoo_finance_url)?;
    let info = YahooFinanceInfo::new(&resp)?;
    Ok(info.get_result_that_day_ja())
}
