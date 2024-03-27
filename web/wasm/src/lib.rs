mod layer;
mod palette;
mod viewport;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct World {
    context: CanvasRenderingContext2d,
    viewport: viewport::Viewport,
    layer_manager: layer::LayerManager,
    palette: palette::Palette,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(bitmap_width: i32, bitmap_height: i32) -> Result<World, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut world = World {
            context,
            viewport: viewport::Viewport::new(),
            layer_manager: layer::LayerManager::new(bitmap_width, bitmap_height),
            palette: palette::Palette::new(),
        };

        world.layer_manager.add_layer();

        Ok(world)
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        for x in 0..(self.layer_manager.width) {
            for y in 0..(self.layer_manager.height) {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for layer in &self.layer_manager.layers {
                    if !layer.visible {
                        continue;
                    }

                    let layer_x = x - layer.offset_x;
                    let layer_y = y - layer.offset_y;

                    // skip if out of bounds
                    if layer_x < 0 || layer_x > layer.width || layer_y < 0 || layer_y > layer.height
                    {
                        continue;
                    }

                    let pixel_index = (x + y * self.layer_manager.width) as usize;
                    let palette_index = layer.pixels[pixel_index];
                    let layer_r = self.palette.colors[palette_index as usize * 3];
                    let layer_g = self.palette.colors[palette_index as usize * 3 + 1];
                    let layer_b = self.palette.colors[palette_index as usize * 3 + 2];

                    match layer.opacity {
                        0 => continue,
                        255 => {
                            r = layer_r;
                            g = layer_g;
                            b = layer_b;
                        }
                        a => {
                            r = (a * (layer_r) + (255 - a) * r) as u8 / 255;
                            g = (a * (layer_b) + (255 - a) * g) as u8 / 255;
                            b = (a * (layer_b) + (255 - a) * b) as u8 / 255;
                        }
                    }
                }

                self.context
                    .set_fill_style(&JsValue::from_str(&format!("rgb({}, {}, {})", r, g, b)));
                self.context.fill_rect(
                    (x + self.viewport.offset.0) as f64 * self.viewport.zoom,
                    (y + self.viewport.offset.1) as f64 * self.viewport.zoom,
                    self.viewport.zoom,
                    self.viewport.zoom,
                );
            }
        }

        self.context.restore();
    }

    #[wasm_bindgen]
    pub fn set_palette_color(&mut self, palette_index: usize, r: u8, g: u8, b: u8) {
        self.palette.colors[palette_index * 3] = r;
        self.palette.colors[palette_index * 3 + 1] = g;
        self.palette.colors[palette_index * 3 + 2] = b;
    }

    #[wasm_bindgen]
    pub fn pan(&mut self, dx: i32, dy: i32) {
        self.viewport.offset.0 += dx;
        self.viewport.offset.1 += dy;
    }

    #[wasm_bindgen]
    pub fn zoom(&mut self, dz: f64) {
        self.viewport.zoom *= dz;
    }
}
