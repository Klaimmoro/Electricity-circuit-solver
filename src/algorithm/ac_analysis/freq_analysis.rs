use std::f32::consts::PI;
use indexmap::IndexMap;
use nalgebra::Complex;
use num_complex::ComplexFloat;
use crate::{
    algorithm::kirchhoff_laws::entities::equation_k::EquationK,
    kernel::{circuit::circuit::Circuit, elements::element_type::ElementType},
};
pub struct FreqAnalysis {
    circuit: Circuit,
    equations: Vec<IndexMap<String,Complex<f32>>>,
    equals: Vec<IndexMap<String,Complex<f32>>>,
    constants: IndexMap<String,Complex<f32>>,
    kirchhoff_equations: Vec<EquationK>,
}
//
//
impl FreqAnalysis {
    ///
    /// New instance [FreqAnalysis]
    pub fn new(circuit: Circuit, kirchhoff_equations: Vec<EquationK>) -> Self {
        Self {
            circuit,
            equations: Vec::new(),
            equals: Vec::new(),
            constants: IndexMap::new(),
            kirchhoff_equations,
        }
    }
    ///
    /// Solving equations by MNA
    pub fn solve(&mut self) {
        self.generate();
        let size = self.equations.len();
        let mut system: Vec<Vec<Complex<f32>>> = Vec::new();
        let mut count = 0;
        for eq in self.equations.clone() {
            let mut tmp: Vec<Complex<f32>> = Vec::new();
            // Добавляем коэффициенты уравнений
            for value in eq.values() {
                tmp.push(*value); // Используем * для разыменования, если Complex реализует Copy
            }
            // Добавляем правую часть уравнения
            tmp.push(self.equals[count][0]); // Получаем значение по индексу
            println!("{:?}",tmp);
            system.push(tmp);
            count+=1;
        }
        for i in 0..size-1 {
            for j in i..size-1 {
                if system[i][i] == ( Complex { re: 0.0, im: 0.0 } ) {
                    continue;
                } else {
                    let factor = system[j + 1][i] / system[i][i];
                    for k in i..size+1 {
                        let tmp = system[i][k].clone();
                        system[j + 1][k] -= factor * tmp;
                    }
                }
            }
        }
        for i in (1..size).rev() {
            if system[i][i] == ( Complex { re: 0.0, im: 0.0 } ) {
                continue;
            } else {
                for j in (1..i+1).rev() {
                    let factor = system[j - 1][i] / system[i][i];
                    for k in (0..size+1).rev() {
                        let tmp = system[i][k].clone();
                        system[j - 1][k] -= factor * tmp;
                    }
                }
            }
        }
        for i in 0..size {
            if system[i][i] == ( Complex { re: 0.0, im: 0.0 } ) {
                println!("Infnitely many solutions");
            }
            else {
                let tmp = system[i][i].clone();
                system[i][size] /= tmp;
                system[i][i] = Complex { re: 1.0, im: 0.0 };
                self.constants[i] = system[i][size];
            }
        }
        for consta in self.constants.clone() {
            println!("{:?}",consta);
        }
    }
    ///
    /// Generating equations by Modified nodal analysis (MNA) from equations by Kirchhofs first law
    fn generate(&mut self) {
        for equation in self.kirchhoff_equations.clone() {
            let mut curr_eq: IndexMap<String,Complex<f32>> = IndexMap::new();
            let curr_equal: IndexMap<String,Complex<f32>> = IndexMap::new();
            for expression in equation.left_side {
                match expression {
                    ElementType::Resistor(resistor) => {
                        if resistor.connection.to != "Ground" {
                            curr_eq
                                .entry(resistor.connection.to.clone())
                                .and_modify(|value| *value += Complex::new(resistor.resistance,0.0))
                                .or_insert(Complex::new(resistor.resistance,0.0));
                            self.constants.entry(resistor.connection.to).or_insert(Complex { re: 0.0, im: 0.0 });
                        }
                        if resistor.connection.from != "Ground" {
                            curr_eq
                                .entry(resistor.connection.from.clone())
                                .and_modify(|value| *value += Complex::new(-resistor.resistance,0.0))
                                .or_insert(Complex::new(-resistor.resistance,0.0));
                            self.constants.entry(resistor.connection.from).or_insert(Complex { re: 0.0, im: 0.0 });
                        }
                    }
                    ElementType::Inductor(inductor) => {
                        let omega = 2.0 * PI * self.circuit.frequency;
                        if inductor.connection.to != "Ground" {
                            curr_eq
                                .entry(inductor.connection.to.clone())
                                .and_modify(|value| *value += Complex::new(0.0,omega * inductor.inductance))
                                .or_insert(Complex::new(0.0,omega * inductor.inductance));
                            self.constants.entry(inductor.connection.to).or_insert(Complex { re: 0.0, im: 0.0 });

                        }
                        if inductor.connection.from != "Ground" {
                            curr_eq
                                .entry(inductor.connection.from.clone())
                                .and_modify(|value| *value += Complex::new(0.0,omega * -inductor.inductance))
                                .or_insert(Complex::new(0.0,omega * -inductor.inductance));
                            self.constants.entry(inductor.connection.from).or_insert(Complex { re: 0.0, im: 0.0 });

                        }
                    }
                    ElementType::Capacitor(capacitor) => {
                        let omega = 2.0 * PI * self.circuit.frequency;
                        if capacitor.connection.to != "Ground" {
                            curr_eq
                                .entry(capacitor.connection.to.clone())
                                .and_modify(|value| *value += 1.0/Complex::new(0.0,omega * capacitor.capacitance))
                                .or_insert(1.0/Complex::new(0.0,omega * capacitor.capacitance));
                            self.constants.entry(capacitor.connection.to).or_insert(Complex { re: 0.0, im: 0.0 });

                        }
                        if capacitor.connection.from != "Ground" {
                            curr_eq
                                .entry(capacitor.connection.from.clone())
                                .and_modify(|value| *value += 1.0/Complex::new(0.0,omega * -capacitor.capacitance))
                                .or_insert(1.0/Complex::new(0.0,omega * -capacitor.capacitance));
                            self.constants.entry(capacitor.connection.from).or_insert(Complex { re: 0.0, im: 0.0 });

                        }
                    }
                    ElementType::VoltageSourceAC(voltage_source_ac) => {
                        let phi = std::f32::consts::PI / voltage_source_ac.voltage;
                        curr_eq
                            .entry(voltage_source_ac.id.clone())
                            .and_modify(|value| *value += Complex::new(1.0,0.0))
                            .or_insert(Complex::new(1.0,0.0));
                        self.equations.push(IndexMap::from_iter([
                            (voltage_source_ac.connection.to.clone(), Complex::new(1.0, 0.0)),
                            (voltage_source_ac.connection.from.clone(), Complex::new(-1.0, 0.0)),
                        ]));
                        let mut tmp: IndexMap<String,Complex<f32>> = IndexMap::new(); tmp.insert(voltage_source_ac.id.clone(), Complex::new(voltage_source_ac.voltage/2.0.powf(1.0 / 2.0) * phi.cos(), 0.0)
                                                                                                                                    + Complex::new(0.0, voltage_source_ac.voltage/2.0.powf(1.0 / 2.0) * phi.sin())
                        );
                        self.constants.entry(voltage_source_ac.id).or_insert(Complex { re: 0.0, im: 0.0 });
                        self.equals.push(tmp);
                    }
                    ElementType::VoltageSourceDC(voltage_source_dc) => {
                        curr_eq
                            .entry(voltage_source_dc.id.clone())
                            .and_modify(|value| *value += Complex::new(1.0,0.0))
                            .or_insert(Complex::new(1.0,0.0));
                        if voltage_source_dc.connection.to != "Ground" && voltage_source_dc.connection.to != "Ground" {
                            self.equations.push(IndexMap::from_iter([
                                (voltage_source_dc.connection.to.clone(), Complex::new(1.0, 0.0)),
                                (voltage_source_dc.connection.from.clone(), Complex::new(-1.0, 0.0)),
                            ]));
                        }
                        else if voltage_source_dc.connection.from != "Ground" && voltage_source_dc.connection.to == "Ground" {
                            self.equations.push(IndexMap::from_iter([
                                (voltage_source_dc.connection.from.clone(), Complex::new(-1.0, 0.0)),
                            ]));
                        } 
                        else if voltage_source_dc.connection.from == "Ground" && voltage_source_dc.connection.to != "Ground" {
                            self.equations.push(IndexMap::from_iter([
                                (voltage_source_dc.connection.to.clone(), Complex::new(-1.0, 0.0)),
                            ]));
                        }
                        let mut tmp: IndexMap<String,Complex<f32>> = IndexMap::new(); tmp.insert(voltage_source_dc.id.clone(), Complex::new(voltage_source_dc.voltage, 0.0));
                        self.constants.entry(voltage_source_dc.id).or_insert(Complex { re: 0.0, im: 0.0 });
                        self.equals.push(tmp);
                    },
                    _ => continue,
                    // ElementType::CurrentSourceAC(current_source_ac) => todo!(),
                    // ElementType::CurrentSourceDC(current_source_dc) => todo!(),
                    // ElementType::Switch(switch) => todo!(),
                }

            }
            if !curr_equal.is_empty() {
                self.equals.push(curr_equal);
            } else {
                let mut tmp: IndexMap<String,Complex<f32>> = IndexMap::new(); tmp.insert("0".to_owned(), Complex::new(0.0,0.0));
                self.equals.push(tmp);
            }
            self.equations.push(curr_eq.clone());
        }
        for eq in &mut self.equations {
            for constant in self.constants.clone() {
                eq.entry(constant.0).or_insert(Complex { re: 0.0, im: 0.0 });
            }   
        }
        for map in &mut self.equations {
            map.sort_by(|k1, _, k2, _| k1.cmp(k2));
        }
        self.constants.sort_keys();
    }
}
