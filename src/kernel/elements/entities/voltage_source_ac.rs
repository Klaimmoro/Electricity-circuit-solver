use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about voltage source AC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoltageSourceAC {
    pub id: String,
    pub frequency: f32,
    pub phase: f32,
    pub voltage: f32,
    pub connection: Connection,
}
//
//
impl VoltageSourceAC {
    ///
    /// Get current currence
    pub fn curr_currence(&mut self, t: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * self.frequency;
        self.voltage * (omega*t + self.phase).sin()
    }
}