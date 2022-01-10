use std::collections::hash_map;
use crate::editor::{shape::*, component, component::components, entity};
use crate::intrinsics::*;

pub fn voltmeter(origin: Point, name: String) -> component::Component {
    let size = Size::new(80.0, 30.0);
    let offset = 27.0;
    let mut properties = hash_map::HashMap::new();
    properties.insert("name", entity::Property::Text(name, true));
    component::Component::new(
        components::Components::Voltmeter,
        "Voltmeter",
        origin,
        size,
        component::color(),
        Shape::new(vec![
            vec![
                Point::new(0.0, size.h),
                Point::new(0.0, size.h / 2.0),
                Point::new(offset, size.h / 2.0),
            ],
            vec![
                Point::new(size.w, size.h),
                Point::new(size.w, size.h / 2.0),
                Point::new(size.w - offset, size.h / 2.0),
            ],
vec![
            Point::new(35.0, 10.0),
            Point::new(40.0, 20.0),
            Point::new(45.0, 10.0),
]
        ], vec![
            Arc::new(Point::new(size.w / 2.0, size.h / 2.0), 13.0, 0.0, 360.0),
        ]),
        vec![
            Point::new(0.0, size.h),
            Point::new(size.w, size.h)
        ],
        properties
    )
}
