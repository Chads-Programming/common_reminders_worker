use reqwest::ClientBuilder;
use worker::*;

mod consts;
mod reminders;

#[event(scheduled)]
async fn main(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    let bot_api_key = env
        .secret("BOT_API_KEY")
        .map(|e: Secret| e.to_string())
        .expect("BOT_API_KEY Secret not found");

    let bot_host = env
        .secret("BOT_HOST")
        .map(|e: Secret| e.to_string())
        .expect("BOT_ENDPOINT var not found");

    let client = ClientBuilder::default()
        .user_agent("Mozilla/5.0")
        .build()
        .expect("Cannot build client reqwest");

    match event.cron().as_str() {
        consts::WALLET_REFILL_EVENT => {
            reminders::wallet_refill::run(&client, &bot_host, &bot_api_key).await
        }
        consts::GREETING_GOOD_NIGHT_EVENT => {
            reminders::good_night::run(&client, &bot_host, &bot_api_key).await
        }
        consts::GREETING_GOOD_MORNING_EVENT => {
            reminders::good_morning::run(&client, &bot_host, &bot_api_key).await
        }
        consts::ENGLISH_DAY_EVENT => {
            reminders::english_day::run(&client, &bot_host, &bot_api_key).await
        }
        _ => {
            console_error!("Missing event");
        }
    };
}
