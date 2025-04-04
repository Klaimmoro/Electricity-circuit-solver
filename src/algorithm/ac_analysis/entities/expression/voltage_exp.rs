use nalgebra::Complex;
use crate::kernel::elements::entities::{voltage_source_ac::VoltageSourceAC, voltage_source_dc::VoltageSourceDC};
#[derive(Clone, Debug)]
///
/// Represent expression for current of AC voltage source
pub struct VoltageExp {
    pub(crate) numerator: Result<VoltageSourceAC,VoltageSourceDC>,
    pub(crate) denumerator: Complex<f32>,
}
