use super::{capacitor_exp::CapacitorExp, inductor_exp::InductorExp, resistor_exp::ResistorExp, voltage_exp::VoltageExp};
#[derive(Clone, Debug)]
pub enum Expression {
    ResistorExp(ResistorExp),
    CapacitorExp(CapacitorExp),
    InductorExp(InductorExp),
    VoltageExp(VoltageExp),
}
