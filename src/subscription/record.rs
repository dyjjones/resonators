use crate::subscription::subscription::Subscription;

struct SubscriptionRecord {
    id: String,
    promise_id: String,
    url: String,
    retry_policy: Option<String>,
    created_on: u64,
    sort_id: u64,
}

impl SubscriptionRecord {
    fn subscription(&self) -> Option<Subscription> {
        if let Some(s) = self.retry_policy {
            let s = &s;
            serde_json::from_str(s).ok()
        } else {
            None
        }
    }
}
