use super::entities::node::Node;
use crate::kernel::elements::element_type::ElementType;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
///
/// Struct to store info about initial circuit
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Circuit {
    pub type_analysis: String,
    pub frequency: f32,
    pub elements: Vec<ElementType>,
    pub nodes: Vec<Node>,
}
//
//
impl Circuit {
    ///
    /// Struct constructor
    /// - `path` - path to json file, where store circuit
    pub fn new(path: PathBuf) -> Self {
        let json_data = fs::read_to_string(&path).expect("Error reading JSON");
        let parsed: Circuit =
            serde_json::from_str(&json_data).expect("Error deserializing JSON in Circuit");
        Self {
            elements: parsed.elements,
            nodes: parsed.nodes,
            type_analysis: parsed.type_analysis,
            frequency: parsed.frequency,
        }
    }
}
