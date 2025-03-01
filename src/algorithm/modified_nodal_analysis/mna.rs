use std::f32::consts::PI;

use nalgebra::Complex;

use crate::{algorithm::kirchhoff_laws::entities::equation_k::EquationK, kernel::{circuit::circuit::Circuit, elements::element_type::ElementType}};
use super::entities::{equation::Equation, expression::{expression::Expression, inductor_exp::InductorExp, resistor_exp::ResistorExp}};
pub struct MNA {
    circuit: Circuit,
    equations: Vec<Equation>,
    kirchhoff_equations: Vec<EquationK>,
}
//
//
impl MNA {
    ///
    /// New instance [MNA]
    pub fn new(circuit: Circuit, kirchhoff_equations: Vec<EquationK>) -> Self {
        Self {
            circuit,
            equations: Vec::new(),
            kirchhoff_equations,
        }
    }
    ///
    /// Generating equations by Modified nodal analysis (MNA) from equations by Kirchhofs first law
    pub fn generate(&mut self) -> Vec<Equation> {
        for equation in self.kirchhoff_equations.clone() {
            let mut curr_eq = Equation::new();
            for expression in equation.left_side {
                match expression {
                    ElementType::Resistor(resistor) => {
                        curr_eq.left_side.push(
                            Expression::ResistorExp(ResistorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == resistor.connection.to)
                                            .unwrap()
                                            .clone(),
                                denumerator: resistor.resistance.into(),
                            })
                        );
                        curr_eq.left_side.push(
                            Expression::ResistorExp(ResistorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == resistor.connection.from)
                                            .unwrap()
                                            .clone(),
                                denumerator: (-resistor.resistance).into(),
                            })
                        );
                    },
                    ElementType::Inductor(inductor) => {
                        let omega = 2.0 * PI * self.circuit.frequency;
                        curr_eq.left_side.push(
                            Expression::InductorExp(InductorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == inductor.connection.to)
                                            .unwrap()
                                            .clone(),
                                denumerator: Complex::new(
                                    0.0,
                                    omega * inductor.inductance
                                ),
                            })
                        );
                        curr_eq.left_side.push(
                            Expression::ResistorExp(ResistorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == inductor.connection.from)
                                            .unwrap()
                                            .clone(),
                                denumerator: -Complex::new(
                                    0.0,
                                    omega * inductor.inductance
                                ),
                            })
                        );
                    },
                    ElementType::Capacitor(capacitor) => {
                        let omega = 2.0 * PI * self.circuit.frequency;
                        curr_eq.left_side.push(
                            Expression::ResistorExp(ResistorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == capacitor.connection.to)
                                            .unwrap()
                                            .clone(),
                                denumerator: 1.0/Complex::new(
                                    0.0,
                                    omega * capacitor.capacitance
                                ),
                            })
                        );
                        curr_eq.left_side.push(
                            Expression::ResistorExp(ResistorExp{
                                numerator: self.circuit.nodes
                                            .iter()
                                            .find(|node| node.id == capacitor.connection.from)
                                            .unwrap()
                                            .clone(),
                                denumerator: -1.0/Complex::new(
                                    0.0,
                                    omega * capacitor.capacitance
                                ),
                            })
                        );
                    },
                    ElementType::VoltageSourceAC(voltage_source_ac) => {

                    },
                    ElementType::VoltageSourceDC(voltage_source_dc) => todo!(),
                    ElementType::CurrentSourceAC(current_source_ac) => todo!(),
                    ElementType::CurrentSourceDC(current_source_dc) => todo!(),
                    ElementType::Switch(switch) => todo!(),
                }
            }
        }
        Vec::new()
    }
}