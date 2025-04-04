use nalgebra::Complex;

use crate::kernel::circuit::entities::node::Node;
#[derive(Clone, Debug)]
///
/// Represent expression for capacitor I_R = (Node_a - Node_b)/R
/// But store info about seperate part
/// Example: Node_a/R - Node_b/R
pub struct ResistorExp {
    pub(crate) numerator: Node,
    pub(crate) denumerator: Complex<f32>,
}
