use crate::intrinsics::*;
use crate::schema::parts::part;
use crate::schema::{layout, parts, properties, props};
use crate::unit::*;

pub fn resistor(name: String) -> part::Part {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let peak_offset = 5.0;
    part::Part::new(
        parts::Typ::Resistor,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
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
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "value" => (properties::Value::Unit(10.0, Unit::Ohm, Prefix::None), true)
        }),
    )
}

pub fn capacitor(name: String) -> part::Part {
    let size = Size::new(50.0, 20.0);
    let offset = 22.0;
    part::Part::new(
        parts::Typ::Capacitor,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
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
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "value" => (properties::Value::Unit(10.0, Unit::Farad, Prefix::None), true),
            "initial_condition" => (properties::Value::Unit(10.0, Unit::Volt, Prefix::None), true)
        }),
    )
}

pub fn inductor(name: String) -> part::Part {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let radius = 5.0;
    part::Part::new(
        parts::Typ::Inductor,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
                vec![
                    vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                    vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
                ],
                vec![
                    layout::shape::Arc::new(Point::new(offset + radius, 10.0), radius, 180.0, 0.0),
                    layout::shape::Arc::new(
                        Point::new(offset + radius * 3.0, 10.0),
                        radius,
                        180.0,
                        0.0,
                    ),
                    layout::shape::Arc::new(
                        Point::new(offset + radius * 5.0, 10.0),
                        radius,
                        180.0,
                        0.0,
                    ),
                ],
            ),
            vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "value" => (properties::Value::Unit(10.0, Unit::Henry, Prefix::None), true),
            "initial_condition" => (properties::Value::Unit(10.0, Unit::Ampere, Prefix::None), true)
        }),
    )
}

pub fn ground(name: String) -> part::Part {
    let size = Size::new(20.0, 15.0);
    let offset = 5.0;
    part::Part::new(
        parts::Typ::Ground,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
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
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), false),
        }),
    )
}

pub fn node(name: String) -> part::Part {
    let size = Size::new(40.0, 14.0);
    part::Part::new(
        parts::Typ::Node,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
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
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), false),
        }),
    )
}
