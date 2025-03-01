use serde::{Deserialize, Serialize};
///
/// Struct to describe element connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub from: String,
    pub to: String,
}