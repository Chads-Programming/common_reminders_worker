use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};

use crate::webhook::Webhook;

pub struct BotWebhook {
    name: &'static str,
    endpoint: String,
    api_key: String,
}

impl BotWebhook {
    pub fn new(name: &'static str, endpoint: String, api_key: String) -> Self {
        Self {
            name,
            endpoint: endpoint.to_string(),
            api_key,
        }
    }
}

impl Webhook for BotWebhook {
    async fn execute(&self) -> Result<(), String> {
        let client = ClientBuilder::default()
            .user_agent("Mozilla/5.0")
            .build()
            .expect("Cannot build client reqwest");

        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );

        let response = client.post(&self.endpoint).headers(headers).send().await;

        if let Err(err) = response {
            return Err(format!("[{}]: {:?}", self.name, err));
        }

        Ok(())
    }

    fn id(&self) -> String {
        self.name.to_string()
    }
}
