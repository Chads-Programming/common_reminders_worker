/*
    Define the endpoints the reminder cronjob resolve
*/
pub const WALLET_REFILL_ENDPOINT: &str = "wallet/refill";
pub const GREETIN_GOOD_NIGHT_ENDPOINT: &str = "greetings/good-night";

/*
    Define events cronjobs here
*/
// Each saturdays at 17pm UTC  -> 12pm in UTC-5
pub const WALLET_REFILL_EVENT: &str = "0 17 * * 6";
// Each day at 3am UTC -> 10pm in UTC-5
pub const GREETIN_GOOD_NIGHT_EVENT: &str = "0 3 * * *";
