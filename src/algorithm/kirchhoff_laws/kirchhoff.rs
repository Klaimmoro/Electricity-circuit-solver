use super::entities::equation_k::EquationK;
use crate::kernel::{circuit::circuit::Circuit, elements::element_type::ElementType};
///
/// Represent's laws of Kirchhoff
#[derive(Clone, Debug)]
pub struct Kirchhoff {
    first_law_eq: Vec<EquationK>,
    second_law_eq: Vec<EquationK>,
    circuit: Circuit,
}
//
//
impl Kirchhoff {
    ///
    /// New instance [Kirchoff]
    pub fn new(circuit: Circuit) -> Self {
        Self {
            first_law_eq: Vec::new(),
            second_law_eq: Vec::new(),
            circuit,
        }
    }
    ///
    /// Generating equations by first law of Kirchhoff
    pub fn first_law(&mut self) -> Vec<EquationK> {
        for node in &self.circuit.nodes {
            let mut curr_equation = EquationK::new();
            let mut curr_equation_to_paint_left_side = String::new();
            let mut curr_equation_to_paint_right_side = String::new();
            for element in node.elements.clone() {
                match element {
                    crate::kernel::elements::element_type::ElementType::Resistor(mut resistor) => {
                        let sign = if resistor.connection.to == node.id {
                            '+'
                        } else {
                            resistor.resistance *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::Resistor(resistor.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}I_{}", sign, resistor.id));
                    }
                    crate::kernel::elements::element_type::ElementType::Inductor(mut inductor) => {
                        let sign = if inductor.connection.to == node.id {
                            '+'
                        } else {
                            inductor.inductance *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::Inductor(inductor.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}I_{}", sign, inductor.id));
                    }
                    crate::kernel::elements::element_type::ElementType::Capacitor(
                        mut capacitor,
                    ) => {
                        let sign = if capacitor.connection.to == node.id {
                            '+'
                        } else {
                            capacitor.capacitance *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::Capacitor(capacitor.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}I_{}", sign, capacitor.id));
                    }
                    crate::kernel::elements::element_type::ElementType::VoltageSourceAC(
                        mut voltage_source_ac,
                    ) => {
                        let sign = if voltage_source_ac.connection.to == node.id {
                            '+'
                        } else {
                            voltage_source_ac.voltage *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::VoltageSourceAC(voltage_source_ac.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}I_{}", sign, voltage_source_ac.id));
                    }
                    crate::kernel::elements::element_type::ElementType::VoltageSourceDC(
                        mut voltage_source_dc,
                    ) => {
                        let sign = if voltage_source_dc.connection.to == node.id {
                            '+'
                        } else {
                            voltage_source_dc.voltage *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::VoltageSourceDC(voltage_source_dc.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}I_{}", sign, voltage_source_dc.id));
                    }
                    crate::kernel::elements::element_type::ElementType::CurrentSourceAC(
                        mut current_source_ac,
                    ) => {
                        let sign = if current_source_ac.connection.to == node.id {
                            '+'
                        } else {
                            current_source_ac.currence *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::CurrentSourceAC(current_source_ac.clone()));
                        curr_equation_to_paint_right_side
                            .push_str(&format!("{}{}", sign, current_source_ac.id));
                    }
                    crate::kernel::elements::element_type::ElementType::CurrentSourceDC(
                        mut current_source_dc,
                    ) => {
                        let sign = if current_source_dc.connection.to == node.id {
                            '+'
                        } else {
                            current_source_dc.currence *= -1.0;
                            '-'
                        };
                        curr_equation
                            .left_side
                            .push(ElementType::CurrentSourceDC(current_source_dc.clone()));
                        curr_equation_to_paint_left_side
                            .push_str(&format!("{}{}", sign, current_source_dc.id));
                    }
                    _ => continue,
                    //crate::kernel::elements::element_type::ElementType::Switch(switch) => {}
                }
            }
            if curr_equation_to_paint_right_side.is_empty() {
                curr_equation_to_paint_right_side.push('0');
            }
            println!(
                "{}={}",
                curr_equation_to_paint_left_side, curr_equation_to_paint_right_side
            );
            self.first_law_eq.push(curr_equation);
        }
        self.first_law_eq.clone()
    }
}
