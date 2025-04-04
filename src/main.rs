use algorithm::{ac_analysis::freq_analysis::FreqAnalysis, kirchhoff_laws::kirchhoff::Kirchhoff};
use kernel::circuit::circuit::Circuit;
use std::path::PathBuf;
mod algorithm;
mod kernel;
fn main() {
    //                      FIRST EXAMPLE DC
    let circuit = Circuit::new(PathBuf::from("src\\examples\\example_1.json"));
    let _first_example =         FreqAnalysis::new(
        circuit.clone(), 
        Kirchhoff::new(circuit.clone())
            .first_law()
    ).solve();
}