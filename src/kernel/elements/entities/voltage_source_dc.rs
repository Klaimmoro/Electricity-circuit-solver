use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about voltage source DC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoltageSourceDC {
    pub id: String,
    pub voltage: f32,
    pub connection: Connection,
}