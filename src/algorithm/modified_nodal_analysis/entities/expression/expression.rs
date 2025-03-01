use super::{capacitor_exp::CapacitorExp, inductor_exp::InductorExp, resistor_exp::ResistorExp};
use crate::kernel::elements::entities::{
    current_source_ac::CurrentSourceAC, current_source_dc::CurrentSourceDC,
    voltage_source_ac::VoltageSourceAC, voltage_source_dc::VoltageSourceDC,
};
#[derive(Clone, Debug)]
pub enum Expression {
    ResistorExp(ResistorExp),
    CapacitorExp(CapacitorExp),
    InductorExp(InductorExp),
    VoltageExpAC(VoltageSourceAC),
    VoltageExpDC(VoltageSourceDC),
    CurrentExpAC(CurrentSourceAC),
    CurrentExpDC(CurrentSourceDC),
}
