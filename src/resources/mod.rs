use crate::error;
use crate::schema::{parts, shape};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

pub struct Layouts {
    pub layouts: HashMap<String, LayoutsResource>,
}

impl Layouts {
    pub fn new() -> Self {
        let mut resources = Self {
            layouts: HashMap::new(),
        };
        resources.add("source", include_str!("../../resources/shapes/source.json"));
        resources.add("lumped", include_str!("../../resources/shapes/lumped.json"));
        resources.add(
            "non_linear",
            include_str!("../../resources/shapes/non_linear.json"),
        );
        resources.add("probe", include_str!("../../resources/shapes/probe.json"));
        resources
    }

    fn add(&mut self, key: &str, data: &str) {
        self.layouts
            .insert(key.to_string(), LayoutsResource::new(data));
    }

    pub fn get(&self, layout: &str) -> Result<parts::Layout, error::Error> {
        let path = layout.split(".").collect::<Vec<&str>>();
        if path.len() == 2 {
            if let Some(resource) = self.layouts.get(path[0]) {
                resource.get(path[1])
            } else {
                Err(Box::new(error::Import::LibNotFound))
            }
        } else {
            Err(Box::new(error::Import::UnexpectedValue))
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct LayoutsResource {
    pub layouts: HashMap<String, parts::Layout>,
    pub bases: HashMap<String, shape::Shape>,
}

impl LayoutsResource {
    pub fn new(content: &str) -> Self {
        let mut resource: Self =
            serde_json::from_str(content).expect("JSON was not well-formatted");
        // For each shape we add the base of the shape to complete it.
        for (_, layout) in resource.layouts.iter_mut() {
            if let Some(base_name) = &layout.base {
                if let Some(base) = resource.bases.get(base_name) {
                    layout.shape.append(base.clone());
                }
            }
        }
        resource
    }

    pub fn get(&self, key: &str) -> Result<parts::Layout, error::Error> {
        if let Some(layout) = self.layouts.get(key) {
            Ok(layout.clone())
        } else {
            Err(Box::new(error::Import::PartNotFound))
        }
    }
}

pub struct Parts {
    pub parts: HashMap<String, PartsResource>,
}

impl Parts {
    pub fn new() -> Self {
        let mut resources = Self {
            parts: HashMap::new(),
        };
        resources.add("lumped", include_str!("../../resources/parts/lumped.json"));
        resources.add("source", include_str!("../../resources/parts/source.json"));
        resources.add("probe", include_str!("../../resources/parts/probe.json"));
        resources.add(
            "non_linear",
            include_str!("../../resources/parts/non_linear.json"),
        );
        resources
    }

    fn add(&mut self, key: &str, data: &str) {
        self.parts.insert(key.to_string(), PartsResource::new(data));
    }

    pub fn get(&self, part: &str) -> Result<parts::Part, error::Error> {
        let path = part.split(".").collect::<Vec<&str>>();
        if path.len() == 2 {
            if let Some(resource) = self.parts.get(path[0]) {
                resource.get(path[1])
            } else {
                Err(Box::new(error::Import::LibNotFound))
            }
        } else {
            Err(Box::new(error::Import::UnexpectedValue))
        }
    }

    pub fn models(&self) -> String {
        self.parts.iter().fold(String::new(), |mut acc, (_, lib)| {
            if let Some(models) = &lib.models {
                models
                    .iter()
                    .for_each(|model| acc.push_str(&format!(".model {} \n", model)));
            }
            acc
        })
    }
}

#[derive(Clone, Deserialize)]
pub struct PartsResource {
    pub title: String,
    pub parts: HashMap<String, parts::Part>,
    pub models: Option<Vec<String>>,
}

impl PartsResource {
    pub fn new(content: &str) -> Self {
        serde_json::from_str(content).expect("JSON was not well-formatted")
    }

    pub fn get(&self, key: &str) -> Result<parts::Part, error::Error> {
        if let Some(part) = self.parts.get(key) {
            Ok(part.clone())
        } else {
            Err(Box::new(error::Import::PartNotFound))
        }
    }
}
