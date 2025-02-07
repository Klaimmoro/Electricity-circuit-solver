use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about current source AC
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSourceAC {
    id: String,
    frequency: f64,
    phase: f64,
    currence: f64,
    connection: Connection,
}