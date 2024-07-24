use reqwest::header::{HeaderMap, HeaderValue};
use webhook::Webhook;
use worker::*;

mod bot;
mod consts;
mod webhook;

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

    let mut manager = webhook::WebhookManager::new();
    let mut common_headers = HeaderMap::new();

    common_headers.append(
        "Authorization",
        HeaderValue::from_str(bot_api_key.as_str()).unwrap(),
    );

    let wallet_refill_wh = bot::BotWebhook::new(
        "[reminders:wallet-refill]",
        format!("{bot_host}/wallet/refill"),
        bot_api_key.clone(),
    );

    let good_night_wh = bot::BotWebhook::new(
        "[reminders:good_night]",
        format!("{bot_host}/reminder/good-night"),
        bot_api_key.clone(),
    );

    let good_morning_wh = bot::BotWebhook::new(
        "[reminders:good_morning]",
        format!("{bot_host}/reminder/good-morning"),
        bot_api_key.clone(),
    );

    let english_day_wh = bot::BotWebhook::new(
        "[reminders:english_day]",
        format!("{bot_host}/reminder/english-day"),
        bot_api_key.clone(),
    );

    manager.register_webhook(&wallet_refill_wh);
    manager.register_webhook(&good_night_wh);
    manager.register_webhook(&good_morning_wh);
    manager.register_webhook(&english_day_wh);

    match event.cron().as_str() {
        consts::WALLET_REFILL_EVENT => manager.dispatch(wallet_refill_wh.id()).await,
        consts::GREETING_GOOD_NIGHT_EVENT => manager.dispatch(good_night_wh.id()).await,
        consts::GREETING_GOOD_MORNING_EVENT => manager.dispatch(good_morning_wh.id()).await,
        consts::ENGLISH_DAY_EVENT => manager.dispatch(english_day_wh.id()).await,
        _ => {
            console_error!("Missing event");
        }
    };
}
