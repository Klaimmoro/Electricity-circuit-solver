use serde::{Deserialize, Serialize};
///
/// Struct to describe element connection
#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    from: String,
    to: String,
}