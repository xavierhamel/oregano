use crate::schema::{parts, parts::part};
use crate::{dom, intrinsics::*, schema};
use std::cell::RefCell;
use std::rc::Rc;

/// Generate the list of components at the left of the editor.
pub fn load() {
    const BUTTON_WIDTH: f64 = 65.0;
    const BUTTON_HEIGHT: f64 = 45.0;
    const WIDTH_STR: &'static str = "65";
    const HEIGHT_STR: &'static str = "45";
    const BUTTON_RATIO: f64 = 0.75;

    categories();
    parts::Typ::iter().for_each(|typ| {
        let container = dom::select(&format!("[data-category=\"{}\"]", typ.category()));
        let part = part::Part::from(typ);
        let part_name = &part.to_string();
        let (canvas, mut ctx) = dom::canvas::new(dom::attributes! {
            "width" => WIDTH_STR,
            "height" => HEIGHT_STR,
        });
        let size = Size::new(
            part.layout.size.w * BUTTON_RATIO,
            part.layout.size.h * BUTTON_RATIO,
        );
        ctx.translate(Point::new(
            (BUTTON_WIDTH - size.w) / 2.0,
            (BUTTON_HEIGHT - size.h) / 2.0,
        ));
        ctx.scale(BUTTON_RATIO);
        part.draw(&ctx);
        dom::append_children(
            &container,
            vec![&dom::create_element(
                "div",
                dom::attributes! { "class" => "dialog-component-button", "data-part" => part_name },
                vec![
                    canvas,
                    dom::form::label::new(part_name, dom::attributes! {}),
                ],
            )],
        );
    });
}

fn categories() {
    let container = dom::select("#menu__component-list");
    parts::Category::iter().for_each(|category| {
        let name = category.to_string();
        dom::append_children(
            &container,
            vec![
                &dom::form::label::new(&name, dom::attributes! {}),
                &dom::create_element(
                    "div",
                    dom::attributes! {
                        "class" => "flex-row dialog-components",
                        "data-category" => &name
                    },
                    vec![],
                ),
            ],
        );
    });
}
