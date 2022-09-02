use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

pub trait WebComponent {
    fn to_dom(&self, document: &Document) -> Result<Element, JsValue>;
}
