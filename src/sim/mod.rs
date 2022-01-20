pub mod circuit;
pub mod dialog;
mod lumped;
mod source;
use crate::schema::{parts, parts::part};
use crate::{dom, error, unit};

pub fn node_name(idx: usize) -> String {
    (0..(idx / 26) + 1)
        .map(|i| ((idx - 26 * i) as u8 + 97u8) as char)
        .collect::<String>()
}

pub fn part_to_string(component: &part::Part) -> Result<String, error::Error> {
    match component.typ {
        parts::Typ::SourceVoltageDc => source::voltage_dc_to_string(component),
        parts::Typ::SourceCurrentDc => source::current_dc_to_string(component),
        parts::Typ::SourceVoltageAc => source::voltage_ac_to_string(component),
        parts::Typ::SourceCurrentAc => source::current_ac_to_string(component),

        parts::Typ::Inductor => lumped::inductor_to_string(component),
        parts::Typ::Resistor => lumped::resistor_to_string(component),
        parts::Typ::Capacitor => lumped::capacitor_to_string(component),
        parts::Typ::Ground | parts::Typ::Node => Ok("".to_string()),

        parts::Typ::Voltmeter => Ok("".to_string()),
    }
}

pub fn probes_to_strings(parts: &Vec<part::Part>) -> Result<Vec<String>, error::Error> {
    let mut probes = Vec::new();
    for part in parts {
        if parts::Typ::Voltmeter == part.typ {
            let connectors = part.connectors()?;
            probes.push(format!("({},{})", connectors[0], connectors[1]));
        }
    }
    Ok(probes)
}

pub fn tran_analysis_to_string(probes: Vec<String>) -> Result<String, error::Error> {
    let probes = probes.iter().fold("".to_string(), |mut string, probe| {
        string.push_str(" v");
        string.push_str(probe);
        string
    });
    Ok(format!(
        "{}\n.tran {}{:?} {}{:?} uic\n.print tran{}\n.end",
        spice_options(),
        dom::form::text_input::value::<String>(dom::select("[name=\"tran__step\"]"))?,
        unit::Prefix::as_array()
            [dom::form::select::value::<usize>(dom::select("[name=\"tran__step-prefix\"]"))?],
        dom::form::text_input::value::<String>(dom::select("[name=\"tran__stop\"]"))?,
        unit::Prefix::as_array()
            [dom::form::select::value::<usize>(dom::select("[name=\"tran__stop-prefix\"]"))?],
        probes
    ))
}

pub fn freq_analysis_to_string(probes: Vec<String>) -> Result<String, error::Error> {
    let data_type =
        dom::form::select::value::<String>(dom::select("[name=\"sim__freq-data-type\"]"))?;
    let probes = probes.iter().fold("".to_string(), |mut string, probe| {
        string.push_str(&format!(" v{}", data_type));
        string.push_str(probe);
        string
    });
    Ok(format!(
        "{}\n.ac {} {} {}{:?} {}{:?}\n.print ac{}\n.end",
        spice_options(),
        dom::form::select::value::<String>(dom::select("[name=\"sim__freq-variation-type\"]"))?,
        dom::form::text_input::value::<String>(dom::select("[name=\"sim__freq-np\"]"))?,
        dom::form::text_input::value::<String>(dom::select("[name=\"sim__freq-fstart\"]"))?,
        unit::Prefix::as_array()
            [dom::form::select::value::<usize>(dom::select("[name=\"sim__freq-fstart-prefix\"]"))?],
        dom::form::text_input::value::<String>(dom::select("[name=\"sim__freq-fstop\"]"))?,
        unit::Prefix::as_array()
            [dom::form::select::value::<usize>(dom::select("[name=\"sim__freq-fstop-prefix\"]"))?],
        probes
    ))
}

fn spice_options() -> &'static str {
    ".options NOACCT"
}
