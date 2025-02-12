use serde::{Deserialize, Serialize};
///
/// Struct to describe element connection
#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
}