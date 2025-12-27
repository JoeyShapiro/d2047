use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get the canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Draw a simple 2048-style tile
    draw_tile(&context, 50.0, 50.0, 100.0, 2)?;
    draw_tile(&context, 170.0, 50.0, 100.0, 4)?;
    draw_tile(&context, 290.0, 50.0, 100.0, 2048)?;

    // Set up keyboard handler
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        web_sys::console::log_1(&format!("Key pressed: {}", event.key()).into());
        // Handle arrow keys for game logic here
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Keep the closure alive

    Ok(())
}

fn draw_tile(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    size: f64,
    value: u32,
) -> Result<(), JsValue> {
    // Background color based on value
    let color = match value {
        2 => "#eee4da",
        4 => "#ede0c8",
        8 => "#f2b179",
        16 => "#f59563",
        2048 => "#edc22e",
        _ => "#cdc1b4",
    };
    
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(x, y, size, size);
    
    // Draw the number
    ctx.set_fill_style(&JsValue::from_str("#776e65"));
    ctx.set_font("bold 32px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    ctx.fill_text(&value.to_string(), x + size / 2.0, y + size / 2.0)?;
    
    Ok(())
}
