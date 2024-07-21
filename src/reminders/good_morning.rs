use crate::consts;
use reqwest::Client;
use worker;

/*
    Send a POST request to bot endpoint in order to activate the greeting message for the morning
    event
*/
pub async fn run(client: &Client, bot_host: &str, bot_api_key: &str) {
    let url = format!("{bot_host}/{}", consts::GREETING_GOOD_MORNING_ENDPOINT);

    let response = client
        .post(url)
        .header("Authorization", bot_api_key)
        .send()
        .await;

    if let Err(err) = response {
        worker::console_error!("[reminders::good_night]: {:?}", err);
    }

    worker::console_log!("[reminders::good_night]: success");
}
