use crate::kernel::circuit::entities::connection::Connection;
use serde::{Deserialize, Serialize};
///
/// Struct to store info about indcutor
#[derive(Debug, Serialize, Deserialize)]
pub struct Inductor {
    id: String,
    inductance: f64,
    connection: Connection,
}