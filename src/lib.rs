extern crate console_error_panic_hook;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

mod dom;
//mod editor;
mod error;
mod events;
mod intrinsics;
//mod plot;
//mod project;
//mod simulation;
mod unit;
mod utils;
mod view;

mod schema;
mod sim;
mod views;

pub const DEBUG: bool = true;
pub const SCHEMA_CANVAS_ID: &'static str = "#canvas";

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn set_up() {
    // This is for better error message in the console on the web.
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    if !DEBUG {
        // let canvas_id = "canvas";
        // view::generate_toolbar();
        // view::generate_simulation();
        // let editor = Rc::new(RefCell::new(editor::Editor::new(canvas_id)));
        // editor.borrow_mut().update();
        // editor::component::dialogs::load_components_dialog(editor.clone());
        // // Adding mouse and keyboard event handlers. It's really ugly how it was added but it is what
        // // it is...
        // events::add_events(editor.clone(), canvas_id);

        // let (x_label, y_labels, series) =
        //     plot::parser::parse_spice_output(plot::test::TEST_OUTPUT_1);
        // editor
        //     .borrow_mut()
        //     .plots
        //     .update_data(series, x_label, y_labels);
        // editor.borrow_mut().plots.add_plot();
    } else {
        views::parts::load();
        view::generate_toolbar();
        view::generate_simulation();
        let schema = Rc::new(RefCell::new(schema::Schema::new().unwrap()));
        schema.borrow_mut().update();
        //editor::component::dialogs::load_components_dialog(schema.clone());
        events::add_events_schema(schema.clone());
    }
}

pub trait IntoSpice {
    fn into_spice(&self) -> Result<String, error::Error>;
}

macro_rules! clog {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

pub(crate) use clog;
