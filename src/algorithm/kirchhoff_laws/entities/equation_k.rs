use crate::kernel::elements::element_type::ElementType;

#[derive(Clone, Debug)]
pub struct EquationK {
    pub left_side: Vec<ElementType>,
    pub right_side: Vec<ElementType>,
}
//
//
impl EquationK {
    ///
    /// New instance [EquationK]
    pub fn new() -> Self {
        Self {
            left_side: Vec::new(),
            right_side: Vec::new(),
        }
    }
}
