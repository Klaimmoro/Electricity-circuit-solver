use nalgebra::Complex;
use crate::kernel::circuit::entities::node::Node;
#[derive(Clone, Debug)]
///
/// Represent expression for capacitor I_C = (Node_a - Node_b)/Z_C
/// But store info about seperate part
/// Example: Node_a/Zc - Node_b/Zc, where Zc = 1/jwc
pub struct CapacitorExp {
    pub(crate) numerator: Node,
    pub(crate) denumerator: Complex<f32>,
}