use crate::editor::component;
use crate::editor::component::*;

/// All components that are shown in the dialog are listed here. If a component is added, also add
/// it to the `to_string` and `generate` methods.
#[derive(Clone, PartialEq, Copy)]
pub enum Components {
    Node,
    
    Resistor,
    Inductor,
    Capacitor,
    Ground,

    SourceVoltageAc,
    SourceVoltageDc,
    SourceCurrentAc,
    SourceCurrentDc,

    Voltmeter,
}

impl Components {
    /// Return a struct of type component::Component populated in a way that is representing the
    /// component correclty. This is a helper function to generate the component and not having to
    /// recreate the component every time.
    pub fn generate(&self, count: usize) -> component::Component {
        match self {
            Components::Node => lumped::node(Point::new(0.0, 0.0), format!("n{}", count)),

            Components::Resistor => lumped::resistor(Point::new(0.0, 0.0), format!("R{}", count)),
            Components::Inductor => lumped::inductor(Point::new(0.0, 0.0), format!("I{}", count)),
            Components::Capacitor => lumped::capacitor(Point::new(0.0, 0.0), format!("C{}", count)),
            Components::Ground => lumped::ground(Point::new(0.0, 0.0), "0".to_string()),

            Components::SourceVoltageAc => source::voltage_ac(Point::new(0.0, 0.0), format!("VAc{}", count)),
            Components::SourceVoltageDc => source::voltage_dc(Point::new(0.0, 0.0), format!("VDc{}", count)),
            Components::SourceCurrentAc => source::current_ac(Point::new(0.0, 0.0), format!("IAc{}", count)),
            Components::SourceCurrentDc => source::current_dc(Point::new(0.0, 0.0), format!("IDc{}", count)),

            Components::Voltmeter => probe::voltmeter(Point::new(0.0, 0.0), format!("P{}", count)),
        }
    }

    pub fn as_sections() -> Vec<(&'static str, Vec<Components>)> {
        vec![
            (
                "Lumped",
                vec![
                    Components::Resistor,
                    Components::Inductor,
                    Components::Capacitor,
                    Components::Ground,
                    Components::Node,
                ]
            ),
            (
                "Sources",
                vec![
                    Components::SourceVoltageAc,
                    Components::SourceVoltageDc,
                    Components::SourceCurrentAc,
                    Components::SourceCurrentDc,
                ]
            ),
            (
                "Probes",
                vec![
                    Components::Voltmeter,
                ]
            )

        ]
    }
}
