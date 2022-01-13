use crate::editor::{component, component::components, property, shape::*};
use crate::intrinsics::*;
use std::collections::BTreeMap;

pub fn voltmeter(origin: Point, name: String) -> component::Component {
    let size = Size::new(50.0, 33.0);
    let offset = 12.0;
    let mut properties = BTreeMap::new();
    properties.insert("name", property::Property::Text(name, true));
    component::Component::new(
        components::Components::Voltmeter,
        "Voltmeter",
        origin,
        size,
        Shape::new(
            vec![
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
                    Point::new(20.0, 8.0),
                    Point::new(25.0, 17.0),
                    Point::new(30.0, 8.0),
                ],
            ],
            vec![Arc::new(Point::new(size.w / 2.0, 13.0), 13.0, 0.0, 360.0)],
        ),
        vec![Point::new(0.0, size.h), Point::new(size.w, size.h)],
        properties,
    )
}
