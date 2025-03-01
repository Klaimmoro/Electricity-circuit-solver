use super::expression::expression::Expression;
#[derive(Clone, Debug)]
pub struct Equation {
    pub left_side: Vec<Expression>,
    pub right_side: Vec<Expression>,
}
//
//
impl Equation {
    ///
    /// New instance [Equation]
    pub fn new() -> Self {
        Self {
            left_side: Vec::new(),
            right_side: Vec::new(),
        }
    }
}