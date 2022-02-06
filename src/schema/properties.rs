use crate::dom::form::text_input;
use crate::schema::props;
use crate::{dom, error, unit};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::{fmt, str};

pub struct PropertiesData(HashMap<String, PropertyData>);
impl PropertiesData {
    pub fn new() -> Self {
        Self(
            serde_json::from_str(include_str!("../../resources/properties.json"))
                .expect("JSON was not well-formatted"),
        )
    }
    pub fn get(&self, key: String) -> Option<&PropertyData> {
        self.0.get(&key)
    }
}

#[derive(Deserialize)]
pub struct PropertyData {
    pub title: String,
    pub description: Option<String>,
}

/// A value which is stored inside a property of a part. A property can be anything like a string,
/// a number or a more complexe data structure (like a number with a unit (Unit)).
#[derive(Clone)]
pub enum Value {
    String(String),
    F64(f64),
    Unit(f64, unit::Unit, unit::Prefix),
}

impl Value {
    /// Return an input or a group of inputs that are necessery to represent this value.
    pub fn into_input(&self, key: &str, is_model: bool) -> Vec<web_sys::Element> {
        let value = self.value_to_string();
        let name = format!("property__{}", key);
        let mut attributes: HashMap<&str, &str> = dom::attributes! {
            "class" => "",
            "data-property" => &key,
            "data-property-type" => self.typ(),
            "name" => &name,
            "value" => &value,
        };
        if is_model {
            attributes.insert("data-is-model", "");
        }
        let value_input = text_input::new(attributes);

        if let Value::Unit(_, unit, prefix) = &self {
            let unit_string = unit.to_string();
            let hidden_name = format!("property__{}-unit", key);
            vec![
                text_input::hidden(dom::attributes! {
                    "class" => "",
                    "value" => &unit_string,
                    "name" => &hidden_name,
                }),
                value_input,
                dom::form::select::create_unit(
                    &format!("property__{}-unit-prefix", key),
                    &unit,
                    prefix.index(),
                ),
            ]
        } else {
            vec![value_input]
        }
    }

    pub fn typ(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::F64(_) => "f64",
            Self::Unit(_, _, _) => "unit",
        }
    }

    pub fn value_to_string(&self) -> String {
        match self {
            Value::String(value) => value.clone(),
            Value::F64(value) => value.to_string(),
            Value::Unit(value, _, _) => value.to_string(),
        }
    }

    fn format_infinity(&self, value: f64) -> String {
        if value == f64::INFINITY {
            String::from("1.8e+308")
        } else {
            value.to_string()
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<web_sys::Element> for Value {
    fn from(input: web_sys::Element) -> Self {
        if let (Ok(f64_value), Some(typ)) = (
            text_input::value::<f64>(input.clone()),
            input.get_attribute("data-property-type"),
        ) {
            match &typ[..] {
                "f64" => {
                    return Value::F64(f64_value);
                }
                "unit" => {
                    if let Some(name) = input.get_attribute("name") {
                        let unit_input = dom::select(&format!("[name=\"{}-unit\"]", name));
                        let unit =
                            text_input::value::<String>(unit_input).expect("`value` is a string");
                        let prefix_value = dom::select(&format!("[name=\"{}-unit-prefix\"]", name));
                        let prefix = dom::form::select::value::<usize>(prefix_value).unwrap();
                        return Value::Unit(
                            f64_value,
                            unit.parse::<unit::Unit>().unwrap(),
                            unit::Prefix::from(prefix),
                        );
                    }
                }
                _ => {}
            }
        }
        Value::String(text_input::value::<String>(input).expect("`value` is a string"))
    }
}

impl str::FromStr for Value {
    type Err = error::Error;

    // This function is used when importing a file. Take a string and converts it to the struct.
    // The format is the following one:
    // `string[text]` or `unit[value|suffix|unit]`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let (Some(start_idx), Some(end_idx)) = (s.find('['), s.find(']')) {
            let data = s[start_idx + 1..end_idx].split("|").collect::<Vec<&str>>();
            match &s[..start_idx] {
                "string" if data.len() == 1 => Ok(Value::String(data[0].to_string())),
                "f64" if data.len() == 1 => Ok(Value::F64(data[0].parse::<f64>()?)),
                "unit" if data.len() == 3 => Ok(Value::Unit(
                    data[0].parse::<f64>()?,
                    data[1].parse::<unit::Unit>()?,
                    data[2].parse::<unit::Prefix>()?,
                )),
                _ => Err(Box::new(error::Import::UnexpectedValue)),
            }
        } else {
            return Err(Box::new(error::Import::MissingToken));
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::F64(value) => write!(f, "{}", self.format_infinity(*value)),
            Value::Unit(value, _, prefix) => {
                write!(f, "{}{:?}", self.format_infinity(*value), prefix)
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(value) => write!(f, "string[{}]", value),
            Value::F64(value) => write!(f, "f64[{}]", value),
            Value::Unit(value, unit, prefix) => write!(f, "unit[{}|{}|{:?}]", value, unit, prefix),
        }
    }
}

/// A property that help define a part of a circuit. A property can be private (only used
/// internaly) or not. The order represent the order that the property should be shown in menus for
/// example.
#[derive(Clone, Deserialize)]
pub struct Property {
    order: usize,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub value: Value,
    #[serde(default = "bool::default")]
    is_visible: bool,
    #[serde(default = "bool::default")]
    pub is_model: bool,
}

impl Property {
    pub fn new(value: Value, is_visible: bool, order: usize, is_model: bool) -> Self {
        Self {
            value: Value::from(value),
            is_visible,
            order,
            is_model,
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    /// Converts the property into an input that can be used in a dialog based on the type of the
    /// property.
    pub fn into_input(&self, name: &str) -> web_sys::Element {
        let mut value_inputs = self.value.into_input(name, self.is_model);
        let input_name = format!("property__{}-is-visible", name);
        value_inputs.push(dom::form::checkbox::new(
            &dom::form::checkbox::dual_icon("eye", "eye-slash"),
            dom::attributes! { "name" => &input_name[..], "class" => "form__toggle-input-dual-icon" },
            self.is_visible,
        ));
        dom::form::group(value_inputs)
    }

    pub fn from_input(key: &str, order: usize) -> Result<Self, error::Error> {
        let value_input = dom::select(&format!("[data-property=\"{}\"]", key));
        let value = Value::from(value_input.clone());
        let is_model = value_input.get_attribute("data-is-model").is_some();
        let is_visible = dom::form::checkbox::value(dom::select(&format!(
            "[name=\"property__{}-is-visible\"]",
            key
        )))
        .unwrap();
        Ok(Property::new(value, is_visible, order, is_model))
    }
}

impl std::str::FromStr for Property {
    type Err = error::Error;

    /// This function is used when importing a file. Take a string and converts it to the struct.
    /// The format is the following one:
    /// `name<text[text];is_visible;order>`
    fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let substring = if let (Some(start_idx), Some(end_idx)) = (s.find('<'), s.find('>')) {
            &s[start_idx + 1..end_idx]
        } else {
            &s[..]
        };
        let data = substring.split(";").collect::<Vec<&str>>();
        if data.len() != 4 {
            return Err(Box::new(error::Import::MissingToken));
        }
        let value = data[0].parse::<Value>()?;
        let is_visible = data[1].parse::<bool>()?;
        let is_model = data[2].parse::<bool>()?;
        let order = data[3].parse::<usize>()?;

        Ok(Property::new(value, is_visible, order, is_model))
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?};{};{};{}",
            self.value, self.is_visible, self.is_model, self.order
        )
    }
}

/// A group of properties that help define a part.
#[derive(Clone, Deserialize)]
pub struct Properties {
    pub properties: HashMap<String, Property>,
}

impl Properties {
    pub fn new(properties: HashMap<String, Property>) -> Self {
        Self { properties }
    }

    pub fn update_from_inputs(&mut self) {
        let mut properties: HashMap<String, Property> = HashMap::new();
        self.properties.iter().for_each(|(key, property)| {
            if let Ok(prop) = Property::from_input(&key, property.order) {
                properties.insert(key.to_string(), prop);
            }
        });
        self.properties = properties;
    }

    pub fn get(&self, key: &str) -> Result<&Property, error::Error> {
        match self.properties.get(key) {
            Some(property) => Ok(property),
            _ => Err(Box::new(error::Sim::PropertyNotFound(0, key.to_string()))),
        }
    }
}

impl IntoIterator for Properties {
    type Item = (String, Property);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut properties = self
            .properties
            .iter()
            .map(|(key, prop)| (key.clone(), prop.clone()))
            .collect::<Vec<(String, Property)>>();
        properties.sort_by_key(|k| k.1.order());
        properties.into_iter()
    }
}

impl std::str::FromStr for Properties {
    type Err = error::Error;

    /// This function is used when importing a file. Take a string and converts it to the struct.
    /// The format is the following one:
    /// `name<text[text];is_visible;order>,value<unit[value];is_visible;order>`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut properties = props::props! {};
        for property in s.split("!") {
            if property.len() > 0 {
                match property.find('<') {
                    Some(idx) => {
                        properties.insert(
                            property[..idx].to_string(),
                            property[idx..].parse::<Property>()?,
                        );
                    }
                    _ => return Err(Box::new(error::Import::MissingToken)),
                }
            }
        }
        Ok(Properties::new(properties))
    }
}

impl fmt::Debug for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        self.properties.iter().for_each(|(key, property)| {
            output.push_str(&format!("{}<{:?}>!", key, property));
        });
        write!(f, "{}", output)
    }
}
