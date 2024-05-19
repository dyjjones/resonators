use super::lock::Lock;

struct LockRecord {
    resource_id: String,
    process_id: String,
    execution_id: String,
    expiry_in_milliseconds: u64,
    expires_at: u64,
}

impl LockRecord {
    fn lock(&self) -> Option<Lock> {
        Some(Lock {
            resource_id: self.resource_id,
            process_id: self.process_id,
            execution_id: self.execution_id,
            expiry_in_milliseconds: self.expiry_in_milliseconds,
            expires_at: self.expires_at,
        })
    }
}
