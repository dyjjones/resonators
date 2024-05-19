use super::notification::Notification;

pub struct NotificationRecord {
    pub id: String,
    pub promise_id: String,
    pub url: String,
    pub retry_policy: String,
    pub time: u64,
    pub attempt: u64,
}

impl NotificationRecord {
    fn notification(&self) -> Option<Notification> {
        let rp = match serde_json::from_str(&self.retry_policy).ok() {
            Some(rp) => rp,
            _ => return None,
        };

        Some(Notification {
            id: self.id,
            promise_id: self.promise_id,
            url: self.url,
            retry_policy: rp,
            time: self.time,
            attempt: self.attempt,
        })
    }
}
