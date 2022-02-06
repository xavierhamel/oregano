use crate::intrinsics::*;
use crate::schema::{parts, shape};

pub fn voltage_ac() -> parts::Layout {
    let size = Size::new(50.0, 30.0);
    let offset = 10.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![
                    Point::new(0.0, size.h / 2.0),
                    Point::new(offset, size.h / 2.0),
                ],
                vec![
                    Point::new(size.w - offset, size.h / 2.0),
                    Point::new(size.w, size.h / 2.0),
                ],
                vec![Point::new(5.0, 7.0), Point::new(5.0, 11.0)],
                vec![Point::new(3.0, 9.0), Point::new(7.0, 9.0)],
            ],
            vec![
                shape::Arc::new(Point::new(size.w / 2.0, 15.0), 15.0, 0.0, 360.0),
                shape::Arc::new(Point::new(size.w / 2.0, 12.0), 3.0, 90.0, 270.0),
                shape::Arc::new(Point::new(size.w / 2.0, 18.0), 3.0, 270.0, 90.0),
            ],
        ),
        vec![Point::new(0.0, 15.0), Point::new(size.w, 15.0)],
    )
}

pub fn voltage_dc() -> parts::Layout {
    let size = Size::new(50.0, 20.0);
    let offset = 22.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
                vec![Point::new(offset, 0.0), Point::new(offset, size.h)],
                vec![
                    Point::new(size.w - offset, 4.0),
                    Point::new(size.w - offset, size.h - 4.0),
                ],
                vec![Point::new(15.0, 2.0), Point::new(15.0, 6.0)],
                vec![Point::new(13.0, 4.0), Point::new(17.0, 4.0)],
            ],
            vec![],
        ),
        vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
    )
}

pub fn current_ac() -> parts::Layout {
    let size = Size::new(50.0, 30.0);
    let offset = 10.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![
                    Point::new(0.0, size.h / 2.0),
                    Point::new(offset, size.h / 2.0),
                ],
                vec![
                    Point::new(size.w - offset, size.h / 2.0),
                    Point::new(size.w, size.h / 2.0),
                ],
                vec![
                    Point::new(18.0, size.h / 2.0),
                    Point::new(32.0, size.h / 2.0),
                ],
                vec![
                    Point::new(25.0, 10.0),
                    Point::new(18.0, 15.0),
                    Point::new(25.0, 20.0),
                ],
            ],
            vec![
                shape::Arc::new(Point::new(size.w / 2.0, 15.0), 15.0, 0.0, 360.0),
                shape::Arc::new(Point::new(3.0, 4.0), 3.0, 90.0, 270.0),
                shape::Arc::new(Point::new(3.0, 10.0), 3.0, 270.0, 90.0),
            ],
        ),
        vec![
            Point::new(0.0, size.h / 2.0),
            Point::new(size.w, size.h / 2.0),
        ],
    )
    // properties::Properties::new(props::props! {
    //     "name" => (properties::Value::String(name.clone()), true),
    //     "offset" => (properties::Value::Unit(0.0, Unit::Ampere, Prefix::None), false),
    //     "amplitude" => (properties::Value::Unit(10.0, Unit::Ampere, Prefix::None), true),
    //     "frequency" => (properties::Value::Unit(1.0, Unit::Hertz, Prefix::None), false),
    //     "delay" => (properties::Value::Unit(0.0, Unit::Second, Prefix::None), false),
    //     "damping_factor" => (properties::Value::Unit(0.0, Unit::Hertz, Prefix::None), false),
    //     "phase" => (properties::Value::Unit(0.0, Unit::Degree, Prefix::None), false),
    // }),
}

pub fn current_dc() -> parts::Layout {
    let size = Size::new(50.0, 30.0);
    let offset = 10.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![
                    Point::new(0.0, size.h / 2.0),
                    Point::new(offset, size.h / 2.0),
                ],
                vec![
                    Point::new(size.w - offset, size.h / 2.0),
                    Point::new(size.w, size.h / 2.0),
                ],
                vec![
                    Point::new(18.0, size.h / 2.0),
                    Point::new(32.0, size.h / 2.0),
                ],
                vec![
                    Point::new(25.0, 10.0),
                    Point::new(18.0, 15.0),
                    Point::new(25.0, 20.0),
                ],
            ],
            vec![shape::Arc::new(
                Point::new(size.w / 2.0, 15.0),
                15.0,
                0.0,
                360.0,
            )],
        ),
        vec![
            Point::new(0.0, size.h / 2.0),
            Point::new(size.w, size.h / 2.0),
        ],
    )
    // properties::Properties::new(props::props! {
    //     "name" => (properties::Value::String(name.clone()), true),
    //     "value" => (properties::Value::Unit(10.0, Unit::Volt, Prefix::None), true)
    // }),
}
