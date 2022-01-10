extern crate console_error_panic_hook;
use std::panic;
use std::rc::Rc;
use std::cell::RefCell;

mod utils;
mod editor;
mod intrinsics;
mod simulation;
mod unit;
mod dom;
mod view;
mod plot;
mod events;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn set_up() {
    // This is for better error message in the console on the web.
    panic::set_hook(Box::new(console_error_panic_hook::hook));


    let canvas_id = "canvas";
    view::generate_toolbar();
    view::generate_simulation();
    let editor = Rc::new(RefCell::new(editor::Editor::new(canvas_id)));
    editor.borrow_mut().update();
    editor::component::dialogs::load_components_dialog(editor.clone());
    // Adding mouse and keyboard event handlers. It's really ugly how it was added but it is what
    // it is...
    events::add_events(editor.clone(), canvas_id);
}