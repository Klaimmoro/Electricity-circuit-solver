use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about voltage source AC
#[derive(Debug, Serialize, Deserialize)]
pub struct VoltageSourceAC {
    id: String,
    frequency: f64,
    phase: f64,
    voltage: f64,
    connection: Connection,
}