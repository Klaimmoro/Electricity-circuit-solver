use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about capacitor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Capacitor {
    pub id: String,
    pub capacitance: f32,
    pub connection: Connection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impedance: Option<f32>,
}
