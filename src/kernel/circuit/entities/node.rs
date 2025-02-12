use crate::kernel::elements::element_type::ElementType;
use serde::{Deserialize, Serialize};
///
/// Struct to describe node of circuit
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub elements: Vec<ElementType>
}