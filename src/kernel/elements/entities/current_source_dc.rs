use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about current source DC
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSourceDC {
    pub id: String,
    pub currence: f32,
    pub connection: Connection,
}