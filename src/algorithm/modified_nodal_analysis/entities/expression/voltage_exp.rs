use nalgebra::Complex;
use crate::kernel::circuit::entities::node::Node;
#[derive(Clone, Debug)]
///
/// Represent expression for current of AC voltage source
pub struct VoltageExp {
    numerator: Node,
    denumerator: Complex<f32>,
}