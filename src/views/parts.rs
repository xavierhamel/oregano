use crate::{dom, intrinsics::*, PARTS};

/// Generate the list of components at the left of the editor.
pub fn load() {
    const BUTTON_WIDTH: f64 = 65.0;
    const BUTTON_HEIGHT: f64 = 45.0;
    const WIDTH_STR: &'static str = "65";
    const HEIGHT_STR: &'static str = "45";
    const BUTTON_RATIO: f64 = 0.75;

    PARTS.parts.iter().for_each(|(category, lib)| {
        new_category(&category[..], &lib.title);
        lib.parts.iter().for_each(|(_, part)| {
            let container = dom::select(&format!("[data-category=\"{}\"]", category));
            let part_id = part.to_string();
            let part_name = match &part.name {
                Some(name) => name,
                _ => &part_id,
            };
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
                    dom::attributes! {
                        "class" => "dialog-component-button",
                        "data-part" => &part_id,
                    },
                    vec![
                        canvas,
                        dom::form::label::new(part_name, dom::attributes! {}),
                    ],
                )],
            );
        })
    });
}

fn new_category(name: &str, title: &str) {
    let id = format!("components__category-{}", name);
    dom::append_children(
        &dom::select("#menu__component-list"),
        vec![
            &dom::create_element(
                "input",
                dom::attributes! {
                    "type" => "checkbox",
                    "style" => "display:none;",
                    "id" => &id,
                },
                vec![],
            ),
            &dom::form::label::new(
                &title,
                dom::attributes! {
                    "class" => "form__label form__label-parts",
                    "for" => &id[..],
                    "style" => "cursor:pointer;margin-top:18px;",
                },
            ),
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
}
