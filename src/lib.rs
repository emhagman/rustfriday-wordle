mod board;
mod dictionary;
mod traits;
mod utils;
mod wasm_utils;
use std::cell::RefCell;
use std::error;
use std::rc::Rc;

use traits::WebComponent;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, Event};
use web_sys::{Document, Element};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // proper errors in console
    console_error_panic_hook::set_once();

    let board = Rc::new(RefCell::new(board::Board::new_wasm("rusty".to_string())));
    let mut our_board = Rc::clone(&board); // 0xff00ab11

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // add text input for guess
    let input: Element = document.create_element("input")?;
    input.set_attribute("placeholder", "guess a word")?;

    let container = Rc::new(RefCell::new(document.create_element("div").unwrap()));
    let dom_board = our_board.borrow().to_dom(&document)?;
    container.borrow_mut().append_child(&dom_board)?;

    let board_ref = Rc::clone(&board);
    let container_ref = Rc::clone(&container);
    let cb = Closure::wrap(Box::new(move |e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();

        if input.value().len() == 5 {
            &board_ref.borrow_mut().guess(&input.value());
            let document = window.document().expect("should have a document on window");
            let dom_board2 = board.borrow().to_dom(&document).unwrap();
            container_ref.borrow_mut().set_text_content("".into());
            container_ref.borrow_mut().append_child(&dom_board2).unwrap();
        }
    }) as Box<dyn FnMut(_)>);
    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;
    cb.forget();

    body.append_child(&container.borrow())?;
    body.append_child(&input)?;

    Ok(())
}
