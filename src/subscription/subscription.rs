use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    id: String,
    promise_id: String,
    url: String,
    retry_policy: RetryPolicy,
    created_on: u64,
    sort_id: u64,
}

impl Display for Subscription {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Subscription(id={}, promise_id={}, url={}, retry_policy={}",
            self.id, self.promise_id, self.url, self.retry_policy
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetryPolicy {
    delay: u64,
    // TODO: is this the number attempted or the max allowed?
    attempts: u64,
}

impl Display for RetryPolicy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "RetryPolicy(delay={}, attempts={})",
            self.delay, self.attempts
        )
    }
}

// TODO: add retry policy approaches:
// * linear
// * exponential backoff

// enum RetryPolicy {
//     LinearUnbound { delay: u64 },
//     Linear { delay: u64, attempts: u64 },
//     ExponentialUnbound,
//     Exponential { attempts: u64 },
// }
