use crate::consts;
use reqwest::Client;
use worker;

/*
    Send a POST request to bot endpoint in order to activate the english day event
*/
pub async fn run(client: &Client, bot_host: &str, bot_api_key: &str) {
    let url = format!("{bot_host}/{}", consts::ENGLISH_DAY_ENDPOINT);

    let response = client
        .post(url)
        .header("Authorization", bot_api_key)
        .send()
        .await;

    if let Err(err) = response {
        worker::console_error!("[reminders::english_day]: {:?}", err);
    }

    worker::console_log!("[reminders::english_day]: success");
}
