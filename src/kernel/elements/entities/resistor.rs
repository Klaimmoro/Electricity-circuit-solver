use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about resistor
#[derive(Debug, Serialize, Deserialize)]
pub struct Resistor {
    id: String,
    resistance: f64,
    connection: Connection,
}