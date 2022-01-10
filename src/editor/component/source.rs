use std::collections::hash_map;
use crate::editor::{shape::*, component, component::components, entity};
use crate::intrinsics::*;
use crate::unit;

pub fn voltage_ac(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 30.0);
    let offset = 25.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("offset", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Volt, true));
    properties.insert("amplitude", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Volt, true));
    properties.insert("frequency", entity::Property::Unit(1.0, unit::Prefix::None, unit::Unit::Hertz, false));
    properties.insert("delay", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Second, false));
    properties.insert("damping_factor", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Hertz, false));
    properties.insert("phase", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Degree, false));
    component::Component::new(
        components::Components::SourceVoltageAc,
        "Voltage AC",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, 15.0),
                Point::new(offset, 15.0),
            ],
            vec![
                Point::new(size.w - offset, 15.0),
                Point::new(size.w, 15.0),
            ],
        ], vec![
            Arc::new(Point::new(size.w / 2.0, 15.0), 15.0, 0.0, 360.0),
            Arc::new(Point::new(size.w / 2.0, 12.0), 3.0, 90.0, 270.0),
            Arc::new(Point::new(size.w / 2.0, 18.0), 3.0, 270.0, 90.0),
        ]),
        vec![
            Point::new(0.0, 15.0),
            Point::new(size.w, 15.0)
        ],
        properties
    )
}

pub fn voltage_dc(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 37.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("value", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Volt, true));
    component::Component::new(
        components::Components::SourceVoltageDc,
        "Voltage DC",
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
            vec![
                Point::new(offset, 0.0),
                Point::new(offset, size.h),
            ],
            vec![
                Point::new(size.w - offset, 4.0),
                Point::new(size.w - offset, size.h - 4.0),
            ],
        ], vec![]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )
}

pub fn current_ac(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 25.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("offset", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Ampere, true));
    properties.insert("amplitude", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Ampere, true));
    properties.insert("frequency", entity::Property::Unit(1.0, unit::Prefix::None, unit::Unit::Hertz, false));
    properties.insert("delay", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Second, false));
    properties.insert("damping_factor", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Hertz, false));
    properties.insert("phase", entity::Property::Unit(0.0, unit::Prefix::None, unit::Unit::Degree, false));
    component::Component::new(
        components::Components::SourceCurrentAc,
        "Current AC",
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
            vec![
                Point::new(33.0, 10.0),
                Point::new(47.0, 10.0)
            ],
            vec![
                Point::new(40.0, 5.0),
                Point::new(33.0, 10.0),
                Point::new(40.0, 15.0),
            ],
        ], vec![
            Arc::new(Point::new(size.w / 2.0, 10.0), 15.0, 0.0, 360.0),
            Arc::new(Point::new(17.0, -1.0), 3.0, 90.0, 270.0),
            Arc::new(Point::new(17.0, 5.0), 3.0, 270.0, 90.0),
        ]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )
}

pub fn current_dc(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 20.0);
    let offset = 25.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    properties.insert("value", entity::Property::Unit(10.0, unit::Prefix::None, unit::Unit::Ampere, true));
    component::Component::new(
        components::Components::SourceCurrentDc,
        "Current DC",
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
            vec![
                Point::new(33.0, 10.0),
                Point::new(47.0, 10.0)
            ],
            vec![
                Point::new(40.0, 5.0),
                Point::new(33.0, 10.0),
                Point::new(40.0, 15.0),
            ],
        ], vec![
            Arc::new(Point::new(size.w / 2.0, 10.0), 15.0, 0.0, 360.0),
        ]),
        vec![
            Point::new(0.0, 10.0),
            Point::new(size.w, 10.0)
        ],
        properties
    )
}
