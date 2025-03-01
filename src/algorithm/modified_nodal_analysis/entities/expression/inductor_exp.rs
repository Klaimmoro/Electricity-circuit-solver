use nalgebra::Complex;

use crate::kernel::circuit::entities::node::Node;
#[derive(Clone, Debug)]
///
/// Represent expression for capacitor I_L = (Node_a - Node_b)/Z_C
/// But store info about seperate part
/// Example: Node_a/Z_L - Node_b/Z_L, where Z_L = jwL
pub struct InductorExp {
    pub(crate) numerator: Node,
    pub(crate) denumerator: Complex<f32>,
}