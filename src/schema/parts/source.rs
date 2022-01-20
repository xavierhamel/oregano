use crate::intrinsics::*;
use crate::schema::parts::part;
use crate::schema::{layout, parts, properties, props};
use crate::unit::*;

pub fn voltage_ac(name: String) -> part::Part {
    let size = Size::new(80.0, 30.0);
    let offset = 25.0;
    part::Part::new(
        parts::Typ::SourceVoltageAc,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
                vec![
                    vec![Point::new(0.0, 15.0), Point::new(offset, 15.0)],
                    vec![Point::new(size.w - offset, 15.0), Point::new(size.w, 15.0)],
                ],
                vec![
                    layout::shape::Arc::new(Point::new(size.w / 2.0, 15.0), 15.0, 0.0, 360.0),
                    layout::shape::Arc::new(Point::new(size.w / 2.0, 12.0), 3.0, 90.0, 270.0),
                    layout::shape::Arc::new(Point::new(size.w / 2.0, 18.0), 3.0, 270.0, 90.0),
                ],
            ),
            vec![Point::new(0.0, 15.0), Point::new(size.w, 15.0)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "offset" => (properties::Value::Unit(0.0, Unit::Volt, Prefix::None), false),
            "amplitude" => (properties::Value::Unit(10.0, Unit::Volt, Prefix::None), true),
            "frequency" => (properties::Value::Unit(1.0, Unit::Hertz, Prefix::None), false),
            "delay" => (properties::Value::Unit(0.0, Unit::Second, Prefix::None), false),
            "damping_factor" => (properties::Value::Unit(0.0, Unit::Hertz, Prefix::None), false),
            "phase" => (properties::Value::Unit(0.0, Unit::Degree, Prefix::None), false),
        }),
    )
}

pub fn voltage_dc(name: String) -> part::Part {
    let size = Size::new(80.0, 20.0);
    let offset = 37.0;
    part::Part::new(
        parts::Typ::SourceVoltageDc,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
                vec![
                    vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                    vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
                    vec![Point::new(offset, 0.0), Point::new(offset, size.h)],
                    vec![
                        Point::new(size.w - offset, 4.0),
                        Point::new(size.w - offset, size.h - 4.0),
                    ],
                ],
                vec![],
            ),
            vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "value" => (properties::Value::Unit(10.0, Unit::Volt, Prefix::None), true)
        }),
    )
}

pub fn current_ac(name: String) -> part::Part {
    let size = Size::new(80.0, 20.0);
    let offset = 25.0;
    part::Part::new(
        parts::Typ::SourceCurrentAc,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
                vec![
                    vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                    vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
                    vec![Point::new(33.0, 10.0), Point::new(47.0, 10.0)],
                    vec![
                        Point::new(40.0, 5.0),
                        Point::new(33.0, 10.0),
                        Point::new(40.0, 15.0),
                    ],
                ],
                vec![
                    layout::shape::Arc::new(Point::new(size.w / 2.0, 10.0), 15.0, 0.0, 360.0),
                    layout::shape::Arc::new(Point::new(17.0, -1.0), 3.0, 90.0, 270.0),
                    layout::shape::Arc::new(Point::new(17.0, 5.0), 3.0, 270.0, 90.0),
                ],
            ),
            vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "offset" => (properties::Value::Unit(0.0, Unit::Ampere, Prefix::None), false),
            "amplitude" => (properties::Value::Unit(10.0, Unit::Ampere, Prefix::None), true),
            "frequency" => (properties::Value::Unit(1.0, Unit::Hertz, Prefix::None), false),
            "delay" => (properties::Value::Unit(0.0, Unit::Second, Prefix::None), false),
            "damping_factor" => (properties::Value::Unit(0.0, Unit::Hertz, Prefix::None), false),
            "phase" => (properties::Value::Unit(0.0, Unit::Degree, Prefix::None), false),
        }),
    )
}

pub fn current_dc(name: String) -> part::Part {
    let size = Size::new(80.0, 20.0);
    let offset = 25.0;
    part::Part::new(
        parts::Typ::SourceCurrentDc,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
                vec![
                    vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                    vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
                    vec![Point::new(33.0, 10.0), Point::new(47.0, 10.0)],
                    vec![
                        Point::new(40.0, 5.0),
                        Point::new(33.0, 10.0),
                        Point::new(40.0, 15.0),
                    ],
                ],
                vec![layout::shape::Arc::new(
                    Point::new(size.w / 2.0, 10.0),
                    15.0,
                    0.0,
                    360.0,
                )],
            ),
            vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true),
            "value" => (properties::Value::Unit(10.0, Unit::Volt, Prefix::None), true)
        }),
    )
}
