use crate::error;
use crate::schema::properties;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Spice(String);

impl Spice {
    fn replace_connectors(&self, mut pattern: String, connectors: &Vec<String>) -> String {
        connectors.iter().enumerate().for_each(|(idx, conn)| {
            pattern = pattern.replace(&format!("{{conn:{}}}", idx), &conn);
        });
        pattern
    }

    fn replace_props(
        &self,
        pattern: String,
        properties: &properties::Properties,
    ) -> Result<String, error::Error> {
        pattern
            .split("{prop")
            .fold(Ok(String::new()), |acc, split| match acc {
                Ok(curr) => match (&split[0..1], split.find("}")) {
                    (sep @ (":" | "?"), Some(end)) => {
                        let key = &split[1..end];
                        match (sep, properties.get(key)) {
                            (_, Ok(prop)) => {
                                Ok(format!("{}{}{}", curr, prop.value, &split[end + 1..]))
                            }
                            ("?", _) => Ok(format!("{}{}", curr, &split[end + 1..].to_string())),
                            (_, Err(error)) => Err(error),
                        }
                    }
                    _ => Ok(format!("{}{}", curr, split)),
                },
                Err(error) => Err(error),
            })
    }

    fn replace_conditional(&self, pattern: String, properties: &properties::Properties) -> String {
        pattern.split("{?").fold(String::new(), |acc, split| {
            match (split.find(":"), split.find("}")) {
                (Some(sep), Some(end)) => {
                    if properties.properties.contains_key(&split[0..sep]) {
                        format!("{}{}{}", acc, &split[sep + 1..end], &split[end + 1..])
                    } else {
                        acc
                    }
                }
                _ => format!("{}{}", acc, split),
            }
        })
    }

    pub fn to_spice(
        &self,
        properties: &properties::Properties,
        connectors: Vec<String>,
    ) -> Result<String, error::Error> {
        let mut pattern = self.0.clone();
        if pattern.len() == 0 {
            return Ok(pattern);
        }
        pattern = self.replace_connectors(pattern, &connectors);
        pattern = self.replace_props(pattern, &properties)?;
        pattern = self.replace_conditional(pattern, &properties);
        Ok(pattern)
    }
}

impl std::str::FromStr for Spice {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, error::Error> {
        Ok(Spice(s.to_string()))
    }
}
