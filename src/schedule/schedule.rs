use std::fmt::{Display, Formatter};

use crate::{
    idempotency::idempotency, kernel::metadata::metadata::Tags,
    promise::promise,
};

pub struct Schedule {
    pub id: String,
    pub description: String,
    pub cron: String,
    pub tags: Tags,
    pub promise_id: String,
    pub promise_timeout: u64,
    pub promise_param: promise::Value,
    pub promise_tags: Tags,
    pub last_run_time: u64,
    pub next_run_time: u64,
    pub idempotency_key: idempotency::Key,
    pub created_on: u64,
    pub sort_id: u64,
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Schedule(id={}, description={}, cron={}, tags={}, \
            promise_id={}, promise_timeout={}, promise_param={}, \
            promise_tags={}, last_run_time={}, next_run_time={}, \
            idempotency_key={}, created_on={})",
            self.id,
            self.description,
            self.cron,
            self.tags,
            self.promise_id,
            self.promise_timeout,
            self.promise_param,
            self.promise_tags,
            self.last_run_time,
            self.next_run_time,
            self.idempotency_key,
            self.created_on
        )
    }
}
