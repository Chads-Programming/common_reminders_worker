/*
    Define the endpoints the reminder cronjob resolve
*/
pub const WALLET_REFILL_ENDPOINT: &str = "wallet/refill";
pub const GREETING_GOOD_NIGHT_ENDPOINT: &str = "reminder/good-night";
pub const GREETING_GOOD_MORNING_ENDPOINT: &str = "reminder/good-morning";
/*
    Define events cronjobs here
*/
// Each saturdays at 17pm UTC  -> 12pm in UTC-5
pub const WALLET_REFILL_EVENT: &str = "0 17 * * 6";
// Each day at 3am UTC -> 10pm in UTC-5
pub const GREETING_GOOD_NIGHT_EVENT: &str = "0 3 * * *";
// Each day at 13pm UTC -> 8am in UTC-5
pub const GREETING_GOOD_MORNING_EVENT: &str = "0 13 * * *";
