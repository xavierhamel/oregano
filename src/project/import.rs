use crate::schema::{parts, wires};
use crate::{clog, error};

enum FileLocation {
    Start,
    Wires,
    Parts,
    Analysis,
}

pub fn from_oregano(input: &str) -> Result<(Vec<wires::Wire>, Vec<parts::Part>), error::Error> {
    let mut wires = Vec::new();
    let mut parts = Vec::new();
    let mut file_location = FileLocation::Start;
    input
        .split(":::")
        .map(|line| {
            match line {
                "[WIRES]" => file_location = FileLocation::Wires,
                "[PARTS]" => file_location = FileLocation::Parts,
                "[ANALYSIS]" => file_location = FileLocation::Analysis,
                _ => match file_location {
                    FileLocation::Wires => {
                        wires.push(line.parse::<wires::Wire>()?);
                    }
                    FileLocation::Parts => {
                        parts.push(line.parse::<parts::Part>()?);
                    }
                    FileLocation::Analysis => {}
                    _ => {}
                },
            };
            Ok(())
        })
        .collect::<Result<(), error::Error>>()?;
    Ok((wires, parts))
}
