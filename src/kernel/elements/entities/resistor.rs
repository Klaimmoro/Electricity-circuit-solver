use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about resistor
#[derive(Debug, Serialize, Deserialize)]
pub struct Resistor {
    pub id: String,
    pub resistance: f32,
    pub connection: Connection,
}