use serde::{Deserialize, Serialize};

use crate::kernel::circuit::entities::connection::Connection;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Switch {
    id: String,
    connection: Connection,
    state: bool,
    transient_time: f32,
}