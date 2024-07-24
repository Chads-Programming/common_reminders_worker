use std::collections::HashMap;

pub trait Webhook {
    async fn execute(&self) -> Result<(), String>;
    fn id(&self) -> String;
}

pub struct WebhookManager<'a, T: Webhook> {
    weebhook_registry: HashMap<String, &'a T>,
}

impl<'a, T> WebhookManager<'a, T>
where
    T: Webhook,
{
    pub fn new() -> Self {
        Self {
            weebhook_registry: HashMap::new(),
        }
    }

    pub fn register_webhook(&mut self, webhook: &'a T) {
        self.weebhook_registry.insert(webhook.id(), webhook);
    }

    pub async fn dispatch(&self, name: String) {
        if let Some(webhook) = self.weebhook_registry.get(name.as_str()) {
            let response = webhook.execute().await;

            if let Err(err) = response {
                worker::console_error!("{err:?}");
            }

            worker::console_log!("[{}]: success", name);
        }

        worker::console_log!("[{}]: success", name);
    }
}
