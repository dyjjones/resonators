use super::task::Task;

struct TaskRecord {
    id: String,
    counter: u64,
    promise_id: String,
    claim_timeout: u64,
    complete_timeout: u64,
    promise_timeout: u64,
    created_on: u64,
    completed_on: u64,
    is_completed: bool,
}

impl TaskRecord {
    fn task(&self) -> Option<Task> {
        Some(Task {
            id: self.id,
            counter: self.counter,
            promise_id: self.promise_id,
            claim_timeout: self.claim_timeout,
            complete_timeout: self.complete_timeout,
            promise_timeout: self.promise_timeout,
            created_on: self.created_on,
            completed_on: self.completed_on,
            is_completed: self.is_completed,
        })
    }
}
