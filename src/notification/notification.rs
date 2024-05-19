use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::subscription::subscription;

#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub promise_id: String,
    pub url: String,
    pub retry_policy: subscription::RetryPolicy,
    pub time: u64,
    pub attempt: u64,
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Notification(id={}, promise_id={}, url={}, \
                retry_policy={}, time={}, attempt={})",
            self.id,
            self.promise_id,
            self.url,
            self.retry_policy,
            self.time,
            self.attempt
        )
    }
}
