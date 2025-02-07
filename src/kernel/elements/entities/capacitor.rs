use serde::{Deserialize, Serialize};
use crate::kernel::circuit::entities::connection::Connection;
///
/// Struct to store info about capacitor
#[derive(Debug, Serialize, Deserialize)]
pub struct Capacitor {
    id: String,
    capacitance: f64,
    connection: Connection,
}