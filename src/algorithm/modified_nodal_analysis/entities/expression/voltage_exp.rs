use crate::kernel::circuit::entities::node::Node;
use nalgebra::Complex;
#[derive(Clone, Debug)]
///
/// Represent expression for current of AC voltage source
pub struct VoltageExp {
    numerator: Node,
    denumerator: Complex<f32>,
}
