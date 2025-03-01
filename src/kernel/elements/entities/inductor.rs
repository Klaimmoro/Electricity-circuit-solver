use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about indcutor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inductor {
    pub id: String,
    pub inductance: f32,
    pub connection: Connection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impedance: Option<f32>,
}