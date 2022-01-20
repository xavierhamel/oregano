use crate::editor::{component::components, entity::Entity, property, wire};
use crate::intrinsics::*;
use std::collections::BTreeMap;

pub enum Err {
    UnexpectedToken,
    MissingValues,
    MissingToken,
}

pub fn from_oregano(input: &str) -> Result<Vec<Box<dyn Entity>>, Err> {
    enum FileLocation {
        Start,
        Wires,
        Components,
        Analysis,
    }
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();
    let mut file_location = FileLocation::Start;
    input
        .lines()
        .map(|line| {
            match line {
                "[WIRES]" => file_location = FileLocation::Wires,
                "[COMPONENTS]" => file_location = FileLocation::Components,
                "[ANALYSIS]" => file_location = FileLocation::Analysis,
                _ => match file_location {
                    FileLocation::Wires => {
                        let values = &line
                            .split(",")
                            .map(|val| utils::parse::<f64>(val))
                            .collect::<Result<Vec<f64>, Err>>()?;
                        if values.len() != 4 {
                            return Err(Err::MissingValues);
                        }
                        entities.push(Box::new(wire::Wire::new(
                            Point::new(values[0], values[1]),
                            Point::new(values[2], values[3]),
                        )));
                    }
                    FileLocation::Components => {
                        let values = line.split(",").collect::<Vec<&str>>();
                        if values.len() < 4 {
                            return Err(Err::MissingValues);
                        }
                        let mut component = components::Components::from_str(values[0]).generate(0);
                        let rotation = utils::parse::<usize>(values[3])?;
                        for _ in 0..rotation {
                            component.rotate();
                        }
                        component.set_origin(Point::new(
                            utils::parse::<f64>(values[1])?,
                            utils::parse::<f64>(values[2])?,
                        ));
                        let mut properties = BTreeMap::new();
                        for idx in 4..values.len() {
                            let property = values[idx].split(":").collect::<Vec<&str>>();
                            if property.len() == 2 {
                                let value = property::Property::from_str(property[1])?;
                                properties.insert(property[0].to_string(), value);
                            }
                        }
                        component.set_properties(properties);
                        entities.push(Box::new(component))
                    }
                    FileLocation::Analysis => {}
                    _ => {}
                },
            };
            Ok(())
        })
        .collect::<Result<(), Err>>()?;
    Ok(entities)
}

pub mod utils {
    use super::Err;

    pub fn parse<T: std::str::FromStr>(input: &str) -> Result<T, Err> {
        if let Ok(value) = input.parse::<T>() {
            Ok(value)
        } else {
            Err(Err::UnexpectedToken)
        }
    }
}
