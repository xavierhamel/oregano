extern crate console_error_panic_hook;
#[macro_use]
extern crate lazy_static;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

mod dom;
//mod editor;
mod error;
mod events;
mod intrinsics;
mod plot;
mod project;
mod resources;
mod unit;
mod view;

mod schema;
mod sim;
mod views;
use crate::schema::properties;

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

lazy_static! {
    pub static ref LAYOUTS: resources::Layouts = resources::Layouts::new();
    pub static ref PARTS: resources::Parts = resources::Parts::new();
    pub static ref PROPS_DATA: properties::PropertiesData = properties::PropertiesData::new();
}

#[wasm_bindgen]
pub fn set_up() {
    // This is for better error message in the console on the web.
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    views::parts::load();
    view::generate_toolbar();
    view::generate_simulation();
    let schema = Rc::new(RefCell::new(schema::Schema::new().unwrap()));
    let plots = Rc::new(RefCell::new(plot::Plots::new()));
    schema.borrow_mut().update();
    events::add_events_schema(schema.clone(), plots.clone());

    let probes = sim::Probes::new(
        vec![sim::Probe {
            spice: String::from(""),
            name: String::from("P0"),
        }],
        vec![],
    );
    let (x_label, y_labels, series) =
        plot::parser::parse_spice_output(&probes, plot::test::TEST_OUTPUT_1);
    plots.borrow_mut().update_data(series, x_label, y_labels);
    plots.borrow_mut().add_plot();
}

pub trait IntoSpice {
    fn into_spice(&self) -> Result<String, error::Error>;
}

macro_rules! clog {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

pub(crate) use clog;
