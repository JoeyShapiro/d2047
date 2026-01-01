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

        // move all tiles in the direction of the arrow key
        let dir = movement.unwrap();
        let mut did_move = true;
        while did_move {
            did_move = false;
            for tile in tiles.iter_mut() {
                let x = (tile.x as i16) + dir.dx;
                let y = (tile.y as i16) + dir.dy;
                let m = (grid_size - 1) as i16;
    
                tile.x = clamp(x, 0, m) as usize;
                tile.y = clamp(y, 0, m) as usize;

                did_move |= tile.x != tile.cx || tile.y != tile.cy;
                tile.cx = tile.x;
                tile.cy = tile.y;
            }
        }

        // get a new random tile
        let mut made = false;
        while !made {
            let new_x = (Math::random() * grid_size as f64).floor() as usize;
            let new_y = (Math::random() * grid_size as f64).floor() as usize;
            if is_tile(&tiles, new_x, new_y) {
                continue;
            }

            let new_value = if Math::random() < 0.9 { 2 } else { 4 };
            tiles.push(Tile::new(new_value, new_x, new_y));
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
    value: u32,
    x: usize,
    y: usize,
    cx: usize,
    cy: usize,
}

impl Tile {
    fn new(value: u32, x: usize, y: usize) -> Self {
        Tile { value, x, y, cx: x, cy: y }
    }
}

fn is_tile(tiles: &Vec<Tile>, x: usize, y: usize) -> bool {
    for tile in tiles.iter() {
        if tile.x == x && tile.y == y {
            return true;
        }
    }
    false
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
