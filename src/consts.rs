/*
    Define events cronjobs here
*/
// Each saturdays at 17pm UTC  -> 12pm in UTC-5
pub const WALLET_REFILL_EVENT: &str = "0 17 * * sat";
// Each day at 3am UTC -> 10pm in UTC-5
pub const GREETING_GOOD_NIGHT_EVENT: &str = "0 3 * * *";
// Each day at 13pm UTC -> 8am in UTC-5
pub const GREETING_GOOD_MORNING_EVENT: &str = "0 13 * * *";
// Each friday at 10pm UTC -> 6am in UTC-5
pub const ENGLISH_DAY_EVENT_FRI: &str = "0 10 * * fri";
// Each wednesday at 10pm UTC -> 6am in UTC-5
pub const ENGLISH_DAY_EVENT_WED: &str = "0 10 * * wed";
