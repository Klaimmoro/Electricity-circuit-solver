use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about voltage source AC
#[derive(Debug, Serialize, Deserialize)]
pub struct VoltageSourceAC {
    pub id: String,
    pub frequency: f32,
    pub phase: f32,
    pub voltage: f32,
    pub connection: Connection,
}