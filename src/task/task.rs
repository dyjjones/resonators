use std::fmt::{Display, Formatter};

pub struct Task {
    pub id: String,
    pub counter: u64,
    pub promise_id: String,
    pub claim_timeout: u64,
    pub complete_timeout: u64,
    pub promise_timeout: u64,
    pub created_on: u64,
    pub completed_on: u64,
    pub is_completed: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Task(id={}, counter={}, promise_id={}, claim_timeout={}, \
                complete_timeout={}, promise_timeout={}, created_on={}, \
                completed_on={}, is_completed={})",
            self.id,
            self.counter,
            self.promise_id,
            self.claim_timeout,
            self.complete_timeout,
            self.promise_timeout,
            self.created_on,
            self.completed_on,
            self.is_completed
        )
    }
}
