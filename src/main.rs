use algorithm::kirchhoff_laws::kirchhoff::Kirchhoff;
use kernel::circuit::circuit::Circuit;
use std::path::PathBuf;
mod algorithm;
mod kernel;
fn main() {
    //                      FIRST EXAMPLE DC
    let first_example =
        Kirchhoff::new(Circuit::new(PathBuf::from("src\\examples\\example_1.json"))).first_law();
    println!("{:?}", first_example);
}
