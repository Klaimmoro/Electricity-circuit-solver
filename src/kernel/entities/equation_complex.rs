use super::expression_complex::ExpressionComplex;
///
/// Struct to describe equation
#[derive(Clone, Debug, PartialEq)]
pub struct EquationComplex {
    pub left_side: Vec<ExpressionComplex>,
    pub right_side: Vec<ExpressionComplex>
}
//
//
impl EquationComplex {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            left_side: Vec::new(),
            right_side: Vec::new()
        }
    }
}