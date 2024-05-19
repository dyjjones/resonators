use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

// TODO: reminder to think about cache expiration
#[derive(Deserialize, Serialize)]
pub struct Lock {
    pub resource_id: String,
    pub process_id: String,
    pub execution_id: String,
    pub expiry_in_milliseconds: u64,
    pub expires_at: u64,
}

impl Display for Lock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Lock(resource_id={}, process_id={}, execution_id={}, \
                expiry_in_seconds={}, expires_at={})",
            self.resource_id,
            self.process_id,
            self.execution_id,
            self.expiry_in_milliseconds,
            self.expires_at
        )
    }
}
