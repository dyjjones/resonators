use crate::idempotency::idempotency;
use crate::promise::promise;
use crate::schedule::schedule::Schedule;
struct ScheduleRecord {
    id: String,
    description: String,
    cron: String,
    // why is this not Tags?
    tags: String,
    promise_id: String,
    promise_timeout: u64,
    promise_param_headers: String,
    promise_param_data: Vec<String>,
    promise_tags: String,
    last_run_time: u64,
    next_run_time: u64,
    idempotency_key: idempotency::Key,
    created_on: u64,
    sort_id: u64,
}

impl ScheduleRecord {
    fn schedule(&self) -> Option<Schedule> {
        let tags = match serde_json::from_str(&self.tags).ok() {
            Some(tags) => tags,
            _ => return None,
        };
        let prom_param_headers =
            match serde_json::from_str(&self.promise_param_headers).ok() {
                Some(h) => h,
                _ => return None,
            };
        let prom_tags = match serde_json::from_str(&self.promise_tags).ok() {
            Some(tags) => tags,
            _ => return None,
        };

        Some(Schedule {
            id: self.id,
            description: self.description,
            cron: self.cron,
            tags,
            promise_id: self.promise_id,
            promise_timeout: self.promise_timeout,
            promise_param: promise::Value {
                headers: prom_param_headers,
                data: self.promise_param_data,
            },
            promise_tags: prom_tags,
            last_run_time: self.last_run_time,
            next_run_time: self.next_run_time,
            idempotency_key: self.idempotency_key,
            created_on: self.created_on,
            sort_id: self.sort_id,
        })
    }
}
