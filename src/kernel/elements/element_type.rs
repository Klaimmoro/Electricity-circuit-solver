use serde::{Deserialize, Serialize};
use super::entities::{capacitor::Capacitor, current_source_ac::CurrentSourceAC, current_source_dc::CurrentSourceDC, inductor::Inductor, resistor::Resistor, voltage_source_ac::VoltageSourceAC, voltage_source_dc::VoltageSourceDC};
///
/// Enum for structurizing element's type's
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ElementType {
    Resistor(Resistor),
    Inductor(Inductor),
    Capacitor(Capacitor),
    VoltageSourceAC(VoltageSourceAC),
    VoltageSourceDC(VoltageSourceDC),
    CurrentSourceAC(CurrentSourceAC),
    CurrentSourceDC(CurrentSourceDC),
}