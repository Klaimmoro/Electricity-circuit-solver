use crate::{algorithm::kirchhoff_laws::entities::equation_k::EquationK, kernel::circuit::circuit::Circuit};

pub struct TransientAnalysis {
    circuit: Circuit,
    kirchhoff_equations: Vec<EquationK>,
    pub state_equations: Vec<String>
}
//
//
impl TransientAnalysis {
    ///
    /// New instance [TransientAnalysis]
    pub fn new(circuit: Circuit, kirchhoff_equations: Vec<EquationK>) -> Self {
        Self {
            state_equations: Vec::new(),
            circuit,
            kirchhoff_equations,
        }
    }
    ///
    /// Generating state equations
    pub fn generate(&mut self) -> Vec<String> {
        for element in self.circuit.elements.clone() {

        }
        self.state_equations.clone()
    }
}