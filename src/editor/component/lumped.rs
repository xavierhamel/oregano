use crate::editor::{component, component::components, property, shape::*};
use crate::intrinsics::*;
use crate::unit;
use std::collections::BTreeMap;

pub fn resistor(origin: Point, name: String) -> component::Component {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let peak_offset = 5.0;
    let mut properties = BTreeMap::new();
    properties.insert("name".to_string(), property::Property::Text(name, true));
    properties.insert(
        "value".to_string(),
        property::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Ohm, true),
    );
    component::Component::new(
        components::Components::Resistor,
        "Resistor",
        origin,
        size,
        Shape::new(
            vec![vec![
                Point::new(0.0, size.h / 2.0),
                Point::new(offset, size.h / 2.0),
                Point::new(peak_offset / 2.0 + offset, 0.0),
                Point::new(peak_offset * 1.5 + offset, size.h),
                Point::new(peak_offset * 2.5 + offset, 0.0),
                Point::new(peak_offset * 3.5 + offset, size.h),
                Point::new(peak_offset * 4.5 + offset, 0.0),
                Point::new(peak_offset * 5.5 + offset, size.h),
                Point::new(size.w - offset, size.h / 2.0),
                Point::new(size.w, size.h / 2.0),
            ]],
            vec![],
        ),
        vec![
            Point::new(0.0, size.h / 2.0),
            Point::new(size.w, size.h / 2.0),
        ],
        properties,
    )
}

pub fn capacitor(origin: Point, name: String) -> component::Component {
    let size = Size::new(50.0, 20.0);
    let offset = 22.0;
    let mut properties = BTreeMap::new();
    properties.insert("name".to_string(), property::Property::Text(name, true));
    properties.insert(
        "value".to_string(),
        property::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Farad, true),
    );
    properties.insert(
        "initial_condition".to_string(),
        property::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Volt, false),
    );
    component::Component::new(
        components::Components::Capacitor,
        "Capacitor",
        origin,
        size,
        Shape::new(
            vec![
                vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                vec![Point::new(offset, 0.0), Point::new(offset, size.h)],
                vec![
                    Point::new(size.w - offset, 0.0),
                    Point::new(size.w - offset, size.h),
                ],
                vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
            ],
            vec![],
        ),
        vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        properties,
    )
}

pub fn inductor(origin: Point, name: String) -> component::Component {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let radius = 5.0;
    let mut properties = BTreeMap::new();
    properties.insert("name".to_string(), property::Property::Text(name, true));
    properties.insert(
        "value".to_string(),
        property::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Henry, true),
    );
    properties.insert(
        "initial_condition".to_string(),
        property::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Ampere, false),
    );
    component::Component::new(
        components::Components::Inductor,
        "Inductor",
        origin,
        size,
        Shape::new(
            vec![
                vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
            ],
            vec![
                Arc::new(Point::new(offset + radius, 10.0), radius, 180.0, 0.0),
                Arc::new(Point::new(offset + radius * 3.0, 10.0), radius, 180.0, 0.0),
                Arc::new(Point::new(offset + radius * 5.0, 10.0), radius, 180.0, 0.0),
            ],
        ),
        vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        properties,
    )
}

pub fn ground(origin: Point, name: String) -> component::Component {
    let size = Size::new(20.0, 15.0);
    let offset = 5.0;
    let mut properties = BTreeMap::new();
    properties.insert("name".to_string(), property::Property::Text(name, false));
    component::Component::new(
        components::Components::Ground,
        "Ground",
        origin,
        size,
        Shape::new(
            vec![
                vec![
                    Point::new(size.w / 2.0, 0.0),
                    Point::new(size.w / 2.0, offset),
                ],
                vec![
                    Point::new(size.w / 2.0, size.h),
                    Point::new(0.0, offset),
                    Point::new(size.w, offset),
                    Point::new(size.w / 2.0, size.h),
                ],
            ],
            vec![],
        ),
        vec![Point::new(size.w / 2.0, 0.0)],
        properties,
    )
}

pub fn node(origin: Point, name: String) -> component::Component {
    let size = Size::new(40.0, 14.0);
    let mut properties = BTreeMap::new();
    properties.insert("name".to_string(), property::Property::Text(name, true));
    component::Component::new(
        components::Components::Node,
        "Node",
        origin,
        size,
        Shape::new(
            vec![
                vec![Point::new(0.0, 7.0), Point::new(10.0, 7.0)],
                vec![
                    Point::new(10.0, 7.0),
                    Point::new(17.0, 0.0),
                    Point::new(33.0, 0.0),
                    Point::new(33.0, 14.0),
                    Point::new(17.0, 14.0),
                    Point::new(10.0, 7.0),
                ],
            ],
            vec![],
        ),
        vec![Point::new(0.0, 7.0)],
        properties,
    )
}
