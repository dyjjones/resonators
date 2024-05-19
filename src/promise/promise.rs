use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::Serialize;

use crate::{idempotency::idempotency, kernel::metadata::metadata::Tags};

#[derive(Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Pending,
    Resolved,
    Rejected,
    #[serde(rename = "REJECTED_CANCELED")]
    Canceled,
    #[serde(rename = "rejected_timedout")]
    Timedout,
}

impl State {
    fn in_mask(&self, mask: State) -> bool {
        *self == mask
    }
}

#[derive(Serialize)]
pub struct Value {
    pub headers: HashMap<String, String>,
    pub data: Vec<String>,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let debug_headers = format!("{:?}", self.headers);
        let joined_data = self.data.join(" ");
        write!(f, "Value(headers={}, data={})", debug_headers, joined_data)
    }
}

#[derive(Serialize)]
struct Promise {
    id: String,
    state: State,
    param: Value,
    value: Value,
    timeout: u64,
    idempotency_key_for_create: idempotency::Key,
    idempotency_key_for_complete: idempotency::Key,
    created_on: u64,
    completed_on: u64,
    tags: Tags,
    sort_id: u64,
}

impl Promise {
    fn get_timed_out_state(&self) -> State {
        let mut completed_state = State::Timedout;
        if let Some(s) = self.tags.get("resonate:timeout") {
            if s == "true" {
                completed_state = State::Resolved
            }
        }
        completed_state
    }
}
