use crate::intrinsics::*;
use crate::schema::parts::part;
use crate::schema::{layout, parts, properties, props};

pub fn voltmeter(name: String) -> part::Part {
    let size = Size::new(50.0, 33.0);
    let offset = 12.0;
    part::Part::new(
        parts::Typ::Voltmeter,
        layout::PartLayout::new(
            Point::new(0.0, 0.0),
            layout::shape::Shape::new(
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
                vec![layout::shape::Arc::new(
                    Point::new(size.w / 2.0, 13.0),
                    13.0,
                    0.0,
                    360.0,
                )],
            ),
            vec![Point::new(0.0, size.h), Point::new(size.w, size.h)],
        ),
        properties::Properties::new(props::props! {
            "name" => (properties::Value::String(name.clone()), true)
        }),
    )
}
