mod kernel;
mod algorithm;
use std::path::PathBuf;
use algorithm::{ac_analysis::ac_analysis::AC, modified_nodal_analysis::modified_nodal_analysis::MNA};   
use kernel::circuit::circuit::Circuit;
fn main() {
    let result = AC::new(Circuit::new(PathBuf::from("src\\examples\\example_1.json"))).solve();
    for res in result.iter() {
        println!("{:?}",res);
    }
    let result = MNA::new(Circuit::new(PathBuf::from("src\\examples\\example_1.json"))).solve();
    for res in result.iter() {
        println!("{:?}",res);
    }
}