mod board;
mod dictionary;
mod utils;
mod wasm_utils;
use std::cell::RefCell;
use std::rc::Rc;

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

    let container = refresh_board(&document, &our_board)?;

    let board_ref = Rc::clone(&board); // 0xee00ab12
    let cb = Closure::wrap(Box::new(move |e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();

        if input.value().len() == 5 {
            console::log_2(&"length is 5".into(), &input.value().into());
            console::log_2(
                &"first row empty?".into(),
                &board_ref.borrow().rows[0][0].to_string().into(),
            );
            &board_ref.borrow_mut().guess(&input.value());
            let document = window.document().expect("should have a document on window");
            let container2 = refresh_board(&document, &board_ref).unwrap();
            let body = document.body().expect("document should have a body");
            body.append_child(&container2).unwrap();
        }

        console::log_2(&"change".into(), &input.value().into());
    }) as Box<dyn FnMut(_)>);
    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;
    cb.forget();

    container.append_child(&input)?;
    body.append_child(&container)?;

    Ok(())
}

fn refresh_board(document: &Document, board: &RefCell<board::Board>) -> Result<Element, JsValue> {
    // Manufacture the element we're gonna append
    let container: Element = document.create_element("div")?;
    container.set_attribute(
        "style",
        "display: flex; flex-direction: column; align-items: flex-start",
    )?;

    let cell_style = "display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 4px; width: 50px; height: 50px; padding: 4px; border: 2px solid #d3d6da;";
    for row in board.borrow().rows.iter() {
        // Each row
        let row_div: Element = document.create_element("div")?;
        row_div.set_attribute("style", "display: flex; flex-direction: row; flex-shrink: 1")?;
        for cell in row.iter() {
            // Each cell
            let cell_div: Element = document.create_element("div")?;
            match cell {
                board::Cell::Gray(_) => {
                    cell_div.set_attribute("style", &format!("{} background-color: #86888a;", cell_style))
                }
                board::Cell::Green(_) => {
                    cell_div.set_attribute("style", &format!("{} background-color: #6aaa64;", cell_style))
                }
                board::Cell::Yellow(_) => {
                    cell_div.set_attribute("style", &format!("{} background-color: #c9b458;", cell_style))
                }
                board::Cell::Empty => cell_div.set_attribute("style", cell_style),
            }?;
            cell_div.set_text_content(Some(&format!("{}", cell)));
            row_div.append_child(&cell_div)?;
        }
        container.append_child(&row_div)?;
    }
    return Ok(container);
}
