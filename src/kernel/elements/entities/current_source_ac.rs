use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about current source AC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrentSourceAC {
    pub id: String,
    pub frequency: f32,
    pub phase: f32,
    pub currence: f32,
    pub connection: Connection,
}
//
//
impl CurrentSourceAC {
    ///
    /// Get current currence
    pub fn curr_currence(&mut self, t: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * self.frequency;
        self.currence * (omega*t + self.phase).sin()
    }
}