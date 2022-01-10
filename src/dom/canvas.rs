use wasm_bindgen::JsCast;
use std::collections::hash_map;
use crate::dom;

pub fn create(attributes: hash_map::HashMap<&str, &str>) -> web_sys::Element {
    dom::create_element("canvas", attributes, vec![])
}

pub fn as_canvas(canvas: web_sys::Element) -> web_sys::HtmlCanvasElement {
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
