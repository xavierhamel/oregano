use std::collections::hash_map;
use crate::editor::{shape::*, component, component::components, entity};
use crate::intrinsics::*;
use crate::unit;

pub fn resistor(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 15.0;
    let peak_offset = 10.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("value", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Ohm, true));
    component::Component::new(
        components::Components::Resistor,
        "Resistor",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, 10.0),
                Point::new(offset, 10.0),
                Point::new(peak_offset / 2.0 + offset, 0.0),
                Point::new(peak_offset * 1.5 + offset, size.h),
                Point::new(peak_offset * 2.5 + offset, 0.0),
                Point::new(peak_offset * 3.5 + offset, size.h),
                Point::new(peak_offset * 4.5 + offset, 0.0),
                Point::new(size.w - offset, 10.0),
                Point::new(size.w, 10.0),
            ],
        ], vec![]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )
}


pub fn capacitor(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 37.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("value", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Farad, true));
    properties.insert("initial_condition", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Volt, false));
    component::Component::new(
        components::Components::Capacitor,
        "Capacitor",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, 10.0),
                Point::new(offset, 10.0),
            ],
            vec![
                Point::new(offset, 0.0),
                Point::new(offset, size.h),
            ],
            vec![
                Point::new(size.w - offset, 0.0),
                Point::new(size.w - offset, size.h),
            ],
            vec![
                Point::new(size.w - offset, 10.0),
                Point::new(size.w, 10.0),
            ],
        ], vec![]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )
}

pub fn inductor(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 16.0;
    let radius = 8.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("value", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Henry, true));
    properties.insert("initial_condition", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Ampere, false));
    component::Component::new(
        components::Components::Inductor,
        "Inductor",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, 10.0),
                Point::new(offset, 10.0),
            ],
            vec![
                Point::new(size.w - offset, 10.0),
                Point::new(size.w, 10.0),
            ],
        ], vec![
            Arc::new(Point::new(offset + radius, 10.0), radius, 180.0, 0.0),
            Arc::new(Point::new(offset + radius * 3.0, 10.0), radius, 180.0, 0.0),
            Arc::new(Point::new(offset + radius * 5.0, 10.0), radius, 180.0, 0.0),
        ]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )

}


pub fn ground(origin: Point, name: String) -> component::Component {
    let size = Size::new(40.0, 20.0);
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, false));
    component::Component::new(
        components::Components::Ground,
        "Ground",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(20.0, 0.0),
                Point::new(20.0, 10.0),
            ],
            vec![
                Point::new(0.0, 10.0),
                Point::new(40.0, 10.0),
            ],
            vec![
                Point::new(7.0, 15.0),
                Point::new(33.0, 15.0),
            ],
            vec![
                Point::new(14.0, 20.0),
                Point::new(26.0, 20.0),
            ],
        ], vec![]),
        vec![
            Point::new(20.0, 0.0),
        ],
        properties
    )
}

pub fn node(origin: Point, name: String) -> component::Component {
    let size = Size::new(40.0, 14.0);
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    component::Component::new(
        components::Components::Node,
        "Node",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, 7.0),
                Point::new(10.0, 7.0)
            ],
            vec![
                Point::new(10.0, 7.0),
                Point::new(17.0, 0.0),
                Point::new(33.0, 0.0),
                Point::new(33.0, 14.0),
                Point::new(17.0, 14.0),
                Point::new(10.0, 7.0),
            ],
        ], vec![]),
        vec![
            Point::new(0.0, 7.0)
        ],
        properties
    )
}
