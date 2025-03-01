use std::path::PathBuf;
use algorithm::kirchhoff_laws::kirchhoff::Kirchhoff;
use kernel::circuit::circuit::Circuit;
mod kernel;
mod algorithm;
fn main() {
    //                      FIRST EXAMPLE DC
    let first_example = Kirchhoff::new(
        Circuit::new(
            PathBuf::from("src\\examples\\example_1.json")
        )
    ).first_law();
    println!("{:?}",first_example);
}