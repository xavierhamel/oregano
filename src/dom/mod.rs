use crate::error;
use std::collections::hash_map;
use wasm_bindgen::JsCast;
pub mod canvas;
pub mod form;
pub mod panel;

/// Return the document of the window, this is used to manipulate the html.
pub fn document() -> web_sys::Document {
    // We could do checks but we have no way to handle them/show them to the user and if this fail,
    // what should we do?
    web_sys::window().unwrap().document().unwrap()
}

/// Select an element and returns it. If the element is not found or another error happens, this
/// function take care of the error.
pub fn select(selector: &str) -> web_sys::Element {
    match document().query_selector(selector).unwrap() {
        Some(element) => element,
        _ => panic!("Élément non trouvé: {}", selector),
    }
}

/// Select all elements and returns them in a vector. If the element is not found or another error
/// happens, this function take care of the error.
pub fn select_all(selector: &str) -> Vec<web_sys::Element> {
    let mut elements = Vec::new();
    let node_list = document().query_selector_all(selector).unwrap();
    for idx in 0..node_list.length() {
        elements.push(
            node_list
                .item(idx)
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .map_err(|_| ())
                .unwrap(),
        );
    }
    elements
}

/// Create an element from the tag, it's attributes and add the children to it. For the
/// `attributes` parameter, use the `attributes!{}` macro, it will be faster than creating and
/// inserting attributes inside a hashmap direclty.
pub fn create_element(
    tag: &str,
    mut attributes: hash_map::HashMap<&str, &str>,
    children: Vec<web_sys::Element>,
) -> web_sys::Element {
    let document = document();
    // TODO: Add error handling. If an error occurs here, show to the user that an error occured.
    let element = document.create_element(tag).unwrap();
    if let Some(inner_html) = attributes.remove("inner_html") {
        element.set_inner_html(inner_html);
    }
    for (attribute, value) in attributes.iter() {
        element.set_attribute(attribute, value).unwrap();
    }
    for child in children.iter() {
        element.append_child(&child).unwrap();
    }
    element
}

/// Append multiple children to a parent container. This function also handle errors as it should.
/// When calling this function, you do not need to worry about errors.
pub fn append_children(parent: &web_sys::Element, children: Vec<&web_sys::Element>) {
    // TODO: We need to add error handling, the reason of this function.
    for child in children {
        parent.append_child(child).unwrap();
    }
}

/// Return an element with the given html as the inner html of the div. This is useful when you
/// need to add text to an element, call this function in the vector of children.
pub fn inner_html(html: &str) -> web_sys::Element {
    let element = create_element("div", hash_map::HashMap::new(), vec![]);
    element.set_inner_html(html);
    element
}

/// This will convert the given `web_sys::Element` to a more specific type of element to access
/// specific functions of an element. For example, convert a `web_sys::Element` to
/// `web_sys::HtmlInputElement` to get access to the value of the element.
pub fn convert<T: wasm_bindgen::JsCast>(element: web_sys::Element) -> Result<T, error::Error> {
    if let Ok(converted) = element.dyn_into::<T>() {
        Ok(converted)
    } else {
        Err(Box::new(error::Dom::ConvertElement))
    }
}

/// Return the inner html that is needed for an icon.
pub fn icon(icon: &str) -> String {
    format!("<i class=\"fas fa-{}\"></i>", icon)
}

/// This macro return a hashmap of attributes to be used with the create_element function found in
/// this crate. Should be used with both str as the key and the value, because it is the type
/// required by the function `create_element`.
macro_rules! attributes {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(dom::attributes!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { dom::attributes!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = dom::attributes!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

pub(crate) use attributes;
