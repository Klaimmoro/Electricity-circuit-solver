use nalgebra::Complex;
///
/// Struct to describe an expression numerator and denominator
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionComplex {
    pub numerator: String,
    pub denominator: Complex<f32>,
}