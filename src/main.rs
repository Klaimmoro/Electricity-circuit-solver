use std::path::PathBuf;

use kernel::circuit::circuit::Circuit;
mod kernel;
fn main() {
    let _circuit: Circuit = Circuit::new(PathBuf::from("src\\examples\\example_1.json"));
}