pub mod circuit;
pub mod dialog;
pub mod verifier;
use crate::schema::{parts, properties};
use crate::{
    dom,
    dom::form::{select, text_input},
    error,
};

pub fn node_name(idx: usize) -> String {
    (0..(idx / 26) + 1)
        .map(|i| ((idx - 26 * i) as u8 + 97u8) as char)
        .collect::<String>()
}

pub struct Probe {
    pub spice: String,
    pub name: String,
}

impl std::convert::TryFrom<&parts::Part> for Probe {
    type Error = error::Error;
    fn try_from(part: &parts::Part) -> Result<Self, error::Error> {
        let name = part.properties.get("name")?.value.clone();
        let spice = match &part.typ[..] {
            "probe.voltmeter" | "lumped.node" => {
                let mut connectors = part.connectors()?;
                // If there is only 1 connector, it's a node and we need to add the second one.
                connectors.push("0".to_string());
                format!("v({},{})", connectors[0], connectors[1])
            }
            "probe.ampermeter" => format!("i(V{})", name),
            _ => return Err(Box::new(error::Internal::Probe)),
        };
        Ok(Probe {
            spice,
            name: name.to_string(),
        })
    }
}

pub struct Probes {
    voltmeters: Vec<Probe>,
    ampermeters: Vec<Probe>,
}

impl Probes {
    pub fn new(voltmeters: Vec<Probe>, ampermeters: Vec<Probe>) -> Self {
        Self {
            voltmeters,
            ampermeters,
        }
    }
}

impl std::ops::Index<usize> for Probes {
    type Output = Probe;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.ampermeters.len() {
            &self.voltmeters[idx]
        } else {
            &self.ampermeters[idx]
        }
    }
}

impl std::convert::TryFrom<&Vec<parts::Part>> for Probes {
    type Error = error::Error;
    fn try_from(parts: &Vec<parts::Part>) -> Result<Self, error::Error> {
        let mut ampermeters = Vec::new();
        let mut voltmeters = Vec::new();
        for part in parts.iter() {
            match &part.typ[..] {
                "probe.voltmeter" | "lumped.node" => voltmeters.push(Probe::try_from(part)?),
                "probe.ampermeter" => ampermeters.push(Probe::try_from(part)?),
                _ => {}
            }
        }
        // We have to do this because of a bug (I think) in ngspice where if you have
        // the probes v(a, 0) v(0, a) it will convert it to v(a) - v(a) instead of have 2 probes you
        // are left with one and it return only 0. Because of that every probe that is inverted with
        // the ground is put first. Because '(0,' comes before anything else alphabeticaly, this works.
        voltmeters.sort_unstable_by_key(|k| k.spice.clone());
        if voltmeters.len() == 0 && ampermeters.len() == 0 {
            Err(Box::new(error::Sim::NoProbe))
        } else {
            Ok(Self {
                voltmeters,
                ampermeters,
            })
        }
    }
}

impl std::fmt::Display for Probes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .ampermeters
            .iter()
            .chain(self.voltmeters.iter())
            .fold(String::new(), |acc, probe| {
                format!("{}{} ", acc, probe.spice)
            });
        write!(f, "{}", out)
    }
}

pub enum Analysis<'probes> {
    Transiant(TransiantAnalysis<'probes>),
    Frequency(FrequencyAnalysis<'probes>),
}

impl<'probes> std::convert::TryFrom<(String, &'probes Probes)> for Analysis<'probes> {
    type Error = error::Error;
    fn try_from(value: (String, &'probes Probes)) -> Result<Self, error::Error> {
        let (typ, probes) = value;
        match &typ[..] {
            "tran" => Ok(Self::Transiant(TransiantAnalysis::try_from(probes)?)),
            "freq" => Ok(Self::Frequency(FrequencyAnalysis::try_from(probes)?)),
            _ => Err(Box::new(error::Sim::UnavailableAnalysis)),
        }
    }
}

impl<'probes> std::fmt::Display for Analysis<'probes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transiant(transiant) => write!(f, "{}", transiant),
            Self::Frequency(frequency) => write!(f, "{}", frequency),
        }
    }
}

pub struct TransiantAnalysis<'probes> {
    probes: &'probes Probes,
    step: properties::Value,
    stop: properties::Value,
}

impl<'probes> std::convert::TryFrom<&'probes Probes> for TransiantAnalysis<'probes> {
    type Error = error::Error;
    fn try_from(probes: &'probes Probes) -> Result<Self, error::Error> {
        let step_input = dom::select("[name=\"tran__step\"]");
        let stop_input = dom::select("[name=\"tran__stop\"]");
        Ok(Self {
            probes,
            step: properties::Value::from(step_input),
            stop: properties::Value::from(stop_input),
        })
    }
}

impl<'probes> std::fmt::Display for TransiantAnalysis<'probes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".options NOACCT\n.tran {} {} uic\n.print tran {}\n.end",
            self.step, self.stop, self.probes
        )
    }
}

pub struct FrequencyAnalysis<'probes> {
    probes: &'probes Probes,
    np: String,
    variation: String,
    start: properties::Value,
    stop: properties::Value,
}

impl<'probes> std::convert::TryFrom<&'probes Probes> for FrequencyAnalysis<'probes> {
    type Error = error::Error;
    fn try_from(probes: &'probes Probes) -> Result<Self, error::Error> {
        let start_input = dom::select("[name=\"sim__freq-fstart\"]");
        let stop_input = dom::select("[name=\"sim__freq-fstop\"]");
        Ok(Self {
            probes,
            variation: select::value::<String>(dom::select("[name=\"sim__freq-variation-type\"]"))?,
            np: text_input::value::<String>(dom::select("[name=\"sim__freq-np\"]"))?,
            start: properties::Value::from(start_input),
            stop: properties::Value::from(stop_input),
        })
    }
}

impl<'probes> std::fmt::Display for FrequencyAnalysis<'probes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".options NOACCT\n.ac {} {} {} {} uic\n.print tran {}\n.end",
            self.variation, self.np, self.start, self.stop, self.probes
        )
    }
}
