use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about current source DC
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSourceDC {
    id: String,
    currence: f64,
    connection: Connection,
}