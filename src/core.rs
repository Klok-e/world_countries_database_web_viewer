use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct OldNew<T: Debug> {
    pub old: T,
    pub new: T,
}
