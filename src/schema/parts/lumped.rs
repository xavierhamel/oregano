use crate::intrinsics::*;
use crate::schema::{parts, shape};

pub fn resistor() -> parts::Layout {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let peak_offset = 5.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
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
    )
}

pub fn node() -> parts::Layout {
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
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
    )
}

pub fn capacitor() -> parts::Layout {
    let size = Size::new(50.0, 20.0);
    let offset = 22.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
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
    )
}

pub fn inductor() -> parts::Layout {
    let size = Size::new(50.0, 10.0);
    let offset = 10.0;
    let radius = 5.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![Point::new(0.0, 10.0), Point::new(offset, 10.0)],
                vec![Point::new(size.w - offset, 10.0), Point::new(size.w, 10.0)],
            ],
            vec![
                shape::Arc::new(Point::new(offset + radius, 10.0), radius, 180.0, 0.0),
                shape::Arc::new(Point::new(offset + radius * 3.0, 10.0), radius, 180.0, 0.0),
                shape::Arc::new(Point::new(offset + radius * 5.0, 10.0), radius, 180.0, 0.0),
            ],
        ),
        vec![Point::new(0.0, 10.0), Point::new(size.w, 10.0)],
    )
}

pub fn ground() -> parts::Layout {
    let size = Size::new(20.0, 15.0);
    let offset = 5.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
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
    )
}

pub fn diode() -> parts::Layout {
    let size = Size::new(20.0, 20.0);
    let offset = 5.0;
    parts::Layout::new(
        Point::new(0.0, 0.0),
        shape::Shape::new(
            vec![
                vec![
                    Point::new(size.w / 2.0, 0.0),
                    Point::new(size.w / 2.0, offset),
                ],
                vec![
                    Point::new(size.w / 2.0, size.h - offset),
                    Point::new(size.w / 2.0, size.h),
                ],
                vec![
                    Point::new(size.w / 2.0, size.h - offset),
                    Point::new(0.0, offset),
                    Point::new(size.w, offset),
                    Point::new(size.w / 2.0, size.h - offset),
                ],
                vec![
                    Point::new(0.0, size.h - offset),
                    Point::new(size.w, size.h - offset),
                ],
            ],
            vec![],
        ),
        vec![
            Point::new(size.w / 2.0, 0.0),
            Point::new(size.w / 2.0, size.h),
        ],
    )
}
