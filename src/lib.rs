mod board;
mod dictionary;
mod utils;
mod wasm_utils;
use wasm_bindgen::prelude::*;
use web_sys::Element;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let mut board = board::Board::new_wasm("rusty".to_string());

    // TODO: add input box, doing this to have initial state
    board.guess("rogue");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let container: Element = document.create_element("div")?;
    container.set_attribute(
        "style",
        "display: flex; flex-direction: column; align-items: flex-start",
    )?;

    for row in board.rows.iter() {
        // Each row
        let row_div: Element = document.create_element("div")?;
        row_div.set_attribute("style", "display: flex; flex-direction: row; flex-shrink: 1")?;
        for cell in row.iter() {
            // Each cell
            let cell_div: Element = document.create_element("div")?;
            match cell {
                board::Cell::Gray(_) => cell_div.set_attribute(
                    "style",
                    "display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 4px; width: 50px; height: 50px; padding: 4px; background-color: #86888a; border: 2px solid #d3d6da",
                ),
                board::Cell::Green(_) => cell_div.set_attribute(
                    "style",
                    "display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 4px; width: 50px; height: 50px; padding: 4px; background-color: #6aaa64; border: 2px solid #d3d6da",
                ),
                board::Cell::Yellow(_) => cell_div.set_attribute(
                    "style",
                    "display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 4px; width: 50px; height: 50px; padding: 4px; background-color: #c9b458; border: 2px solid #d3d6da",
                ),
                board::Cell::Empty => cell_div.set_attribute(
                    "style",
                    "display: flex; align-items: center; justify-content: center; font-size: 24px; margin: 4px; width: 50px; height: 50px; padding: 4px; border: 2px solid #d3d6da",
                ),
            }?;
            cell_div.set_text_content(Some(&format!("{}", cell)));
            row_div.append_child(&cell_div)?;
        }
        container.append_child(&row_div)?;
    }

    body.append_child(&container)?;

    Ok(())
}
