use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Serialize, PartialEq)]
pub struct Key(String);
impl Key {
    fn eq(&self, other: Key) -> bool {
        *self == other
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
