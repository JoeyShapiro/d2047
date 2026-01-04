use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use web_sys::js_sys::Math;

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
    let mut tiles: Vec<Tile> = Vec::new();

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

        let dir = movement.unwrap();

        // sort tiles based on movement direction
        tiles.sort_by(|a, b| {
            if dir.dx == 1 {
                // moving right
                b.x.cmp(&a.x)
            } else if dir.dx == -1 {
                // moving left
                a.x.cmp(&b.x)
            } else if dir.dy == 1 {
                // moving down
                b.y.cmp(&a.y)
            } else {
                // moving up
                a.y.cmp(&b.y)
            }
        });

        // do merging first to stop gaps
        for i in 0..tiles.len() {
            let x = (tiles[i].x as i16 + dir.dx) as usize;
            let y = (tiles[i].y as i16 + dir.dy) as usize;
            
            let value = tiles[i].value;
            if let Some(other) = tiles.iter_mut().find(|t| t.x == x && t.y == y && t.value == value) {
                // merge
                other.value *= 2;
                tiles[i].value = 0; // mark for removal
            }
        }

        // now move all tiles in the direction of the arrow key
        let mut did_move = true;
        while did_move {
            did_move = false;
            for i in 0..tiles.len() {
                let m = (grid_size - 1) as i16;
                
                let x = clamp((tiles[i].x as i16) + dir.dx, 0, m) as usize;
                let y = clamp((tiles[i].y as i16) + dir.dy, 0, m) as usize;

                // check for collision
                if tiles.iter().enumerate().any(|(j, t)| j != i && t.value != 0 && t.x == x && t.y == y) {
                    continue;
                }

                did_move |= tiles[i].x != x || tiles[i].y != y;
                tiles[i].x = x;
                tiles[i].y = y;
            }
        }

        // remove merged tiles
        tiles.retain(|tile| tile.value != 0);

        // get a new random tile
        let mut made = false;
        while !made {
            let x = (Math::random() * grid_size as f64).floor() as usize;
            let y = (Math::random() * grid_size as f64).floor() as usize;
            if tiles.iter().any(|t| t.x == x && t.y == y) {
                continue;
            }

            let value = if Math::random() < 0.9 { 2 } else { 4 };
            tiles.push(Tile { x, y, value });
            made = true
        }

        // Redraw tiles
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for tile in tiles.iter() {
            let x = tile.x as f64 * tile_size + 50.0;
            let y = tile.y as f64 * tile_size + 50.0;
            draw_tile(&context, x, y, tile_size, tile.value).unwrap();
        }
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Keep the closure alive

    Ok(())
}

struct Tile {
    x: usize,
    y: usize,
    value: u32,
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
        2 => "#6272a4",
        4 => "#8be9fd",
        8 => "#50fa7b",
        16 => "#ffb86c",
        32 => "#ff79c6",
        64 => "#bd93f9",
        128 => "#ff5555",
        256 => "#f1fa8c",
        512 => "#44475a",
        1024 => "#44475a",
        2048 => "#44475a",
        _ => "#ff00ff",
    };
    
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(x, y, size, size);
    
    // Draw the number
    ctx.set_fill_style(&JsValue::from_str("#f8f8f2"));
    ctx.set_font("bold 32px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    ctx.fill_text(&value.to_string(), x + size / 2.0, y + size / 2.0)?;
    
    Ok(())
}
