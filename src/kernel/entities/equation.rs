use super::expression::Expression;
///
/// Struct to describe equation
#[derive(Clone, Debug, PartialEq)]
pub struct Equation {
    pub left_side: Vec<Expression>,
    pub right_side: Vec<Expression>
}
//
//
impl Equation {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            left_side: Vec::new(),
            right_side: Vec::new()
        }
    }
}