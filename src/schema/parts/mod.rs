use std::fmt;
pub mod lumped;
pub mod part;
pub mod probe;
pub mod source;
use crate::schema::{mouse, parts, properties};
use crate::{error, views};

pub struct Parts {
    pub parts: Vec<part::Part>,
    pub hovered: Vec<usize>,
    pub selected: Vec<usize>,
}

impl Parts {
    pub fn new() -> Self {
        let parts = vec![part::Part::from(parts::Typ::Voltmeter)];
        Self {
            parts,
            selected: Vec::new(),
            hovered: Vec::new(),
        }
    }

    pub fn add(&mut self, mut part: part::Part) {
        part.state = part::State::Floating;
        self.parts.push(part);
    }

    pub fn update(&mut self, mouse: &mouse::Mouse) {
        let mut selected = Vec::new();
        let mut hovered = Vec::new();
        self.update_selected();
        if mouse.action != mouse::Action::DrawWire {
            self.parts.iter_mut().enumerate().for_each(|(idx, part)| {
                part.mouse_updated(mouse);
                if part.state.is_selected() {
                    selected.push(idx);
                }
                if part.state.is_hovered() {
                    hovered.push(idx);
                }
            });
        }
        self.selected = selected;
        self.hovered = hovered;
        if mouse.state == mouse::State::Down && self.selected.len() == 1 {
            views::properties::update(&self.parts[self.selected[0]]);
        }
        if self.selected.len() != 1 {
            views::properties::empty();
        }
    }

    pub fn update_selected(&mut self) {
        if self.selected.len() == 1 {
            self.parts[self.selected[0]].properties.update_from_inputs();
        }
    }

    pub fn unselect(&mut self) {
        for idx in self.selected.iter() {
            self.parts[*idx].state.set_selected(false);
        }
        self.parts = self
            .parts
            .iter()
            .filter(|part| part.state != part::State::Floating)
            .map(|part| part.clone())
            .collect::<Vec<part::Part>>();
        self.selected = Vec::new();
    }

    pub fn delete(&mut self) {
        self.parts = self
            .parts
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.selected.contains(idx))
            .map(|(_, part)| part.clone())
            .collect::<Vec<part::Part>>();
        self.selected = Vec::new();
    }

    pub fn rotate(&mut self) {
        for idx in self.selected.iter() {
            self.parts[*idx].layout.rotate();
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, part::Part> {
        self.parts.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, part::Part> {
        self.parts.iter()
    }

    pub fn hovered(&self) -> PartsIter {
        PartsIter::new(&self.parts, &self.hovered)
    }

    pub fn selected(&self) -> PartsIter {
        PartsIter::new(&self.parts, &self.selected)
    }
}

pub struct PartsIter<'a> {
    parts: &'a Vec<part::Part>,
    items: &'a Vec<usize>,
    index: usize,
}

impl<'a> PartsIter<'a> {
    pub fn new(parts: &'a Vec<part::Part>, items: &'a Vec<usize>) -> Self {
        Self {
            parts,
            items,
            index: 0,
        }
    }
}

impl<'a> Iterator for PartsIter<'a> {
    type Item = &'a part::Part;
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.len() >= self.index {
            None
        } else {
            let item = &self.parts[self.items[self.index]];
            self.index += 1;
            Some(item)
        }
    }
}

#[derive(Clone, Copy)]
pub enum Category {
    Lumped,
    Source,
    Probe,
}

impl Category {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Lumped, Self::Source, Self::Probe].iter().copied()
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Lumped => "Lumped",
            Self::Source => "Source",
            Self::Probe => "Probe",
        };
        write!(f, "{}", out)
    }
}

/// This macro will implement all necessary functions for a type of part. Just add it to the list
/// bellow this macro!
macro_rules! typs {
    ($($key:ident => (str: $value:expr, cat: $category:ident)),*) => {
        #[derive(Clone, Copy, PartialEq)]
        pub enum Typ {
            $($key,)*
        }
        impl Typ {
            pub fn category(&self) -> Category {
                match self {
                    $(Typ::$key => Category::$category,)*
                }
            }
            pub fn iter() -> impl Iterator<Item = Typ> {
                [
                    $(Typ::$key,)*
                ].iter().copied()
            }
        }
        impl std::str::FromStr for Typ {
            type Err = error::Error;
            fn from_str(s: &str) -> Result<Typ, error::Error> {
                match s {
                    $($value => Ok(Self::$key),)*
                    _ => Err(Box::new(error::Internal::Parse))
                }
            }
        }
        impl fmt::Display for Typ {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let out = match self {
                    $(Typ::$key => $value,)*
                };
                write!(f, "{}", out)
            }
        }
    };
}

typs! {
    Node => (str: "node", cat: Lumped),
    Resistor => (str: "resistor", cat: Lumped),
    Inductor => (str: "inductor", cat: Lumped),
    Capacitor => (str: "capacitor", cat: Lumped),
    Ground => (str: "ground", cat: Lumped),
    SourceVoltageAc => (str: "source_voltage_ac", cat: Source),
    SourceVoltageDc => (str: "source_voltage_dc", cat: Source),
    SourceCurrentAc => (str: "source_current_ac", cat: Source),
    SourceCurrentDc => (str: "source_current_dc", cat: Source),
    Voltmeter => (str: "voltmeter", cat: Probe)
}
