use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about current source AC
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSourceAC {
    pub id: String,
    pub frequency: f32,
    pub phase: f32,
    pub currence: f32,
    pub connection: Connection,
}