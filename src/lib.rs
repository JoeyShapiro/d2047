use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use web_sys::js_sys::Math;
use std::collections::HashMap;

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

    let tile_size = 100.0;
    let grid_size = 4;
    // not proper to do this, but it is easier
    // feel wrong, dont need anything fancy. can do proper collision detection for something else
    // this is grid based, so just check if something is there, no need to iterate a list 2 or 3 times
    // now i dont need multiple list checks or frames/updates or rewinds
    let mut tiles = HashMap::new();
    tiles.insert((0, 0), 2);

    // Set up keyboard handler
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        // web_sys::console::log_1(&format!("Key pressed: {}", event.key()).into());
        let movement = match event.key().as_str() {
            "ArrowUp" => {
                Some(Movement{dx: 0, dy: -1})
            },
            "ArrowDown" => {
                Some(Movement{dx: 0, dy: 1})
            },
            "ArrowLeft" => {
                Some(Movement{dx: -1, dy: 0})
            },
            "ArrowRight" => {
                Some(Movement{dx: 1, dy: 0})
            },
            _ => None,
        };

        if movement.is_none() {
            return;
        }

        // move all tiles in the direction of the arrow key
        let dir = movement.unwrap();
        let mut did_move = true;
        while did_move {
            did_move = false;
            for (pos, value) in tiles.clone().iter() {
                let m = (grid_size - 1) as i16;
    
                let x = clamp((pos.0 as i16) + dir.dx, 0, m) as usize;
                let y = clamp((pos.1 as i16) + dir.dy, 0, m) as usize;

                did_move |= pos.0 != x || pos.1 != y;
                // you cant move a box when youre standing in it
                // but this is technically a clone
                tiles.remove(pos);
                tiles.insert((x, y), *value);
            }
        }

        // now check for merges
        // went back and forth on this and a list. but this is simple and clean

        // get a new random tile
        let mut made = false;
        while !made {
            let new_x = (Math::random() * grid_size as f64).floor() as usize;
            let new_y = (Math::random() * grid_size as f64).floor() as usize;
            if tiles.contains_key(&(new_x, new_y)) {
                continue;
            }

            let new_value = if Math::random() < 0.9 { 2 } else { 4 };
            tiles.insert((new_x, new_y), new_value);
            made = true
        }

        // Redraw tiles
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for (pos, value) in tiles.iter() {
            let x = pos.0 as f64 * tile_size + 50.0;
            let y = pos.1 as f64 * tile_size + 50.0;
            draw_tile(&context, x, y, tile_size, *value).unwrap();
        }
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Keep the closure alive

    Ok(())
}

struct Movement {
    dx: i16,
    dy: i16,
}

fn clamp(value: i16, min: i16, max: i16) -> i16 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
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
