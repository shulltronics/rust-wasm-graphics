use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb888,
    geometry::{OriginDimensions, Size},
    draw_target::DrawTarget
};

use wasm_bindgen::{
    JsCast,
    Clamped,
};

use web_sys::CanvasRenderingContext2d;

// This is part of the html canvas definition (right?)
const BYTES_PER_PIXEL: usize = 4;

pub struct CanvasDisplay {
    pub width: u32,
    pub height: u32,
    buffer: Vec<u8>,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl CanvasDisplay {
    pub fn new(c: &web_sys::HtmlCanvasElement) -> CanvasDisplay {
        Self {
            width: c.width(),
            height: c.height(),
            buffer: vec![0u8; (c.width() * c.height()) as usize * BYTES_PER_PIXEL],
            ctx: c.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
        }
    }
}

impl DrawTarget for CanvasDisplay {
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        // A buffer for our pixels
        for Pixel(coord, color) in pixels.into_iter() {
            // constrain coordinate to canvas area
            let (x, y) = (coord.x, coord.y);
            if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
                continue;
            } else {
                // calculate the buffer index
                let idx: usize = ((x as usize)*BYTES_PER_PIXEL) + (y as usize)*((self.width as usize)*BYTES_PER_PIXEL);
                let px: [u8; 4] = [color.r(), color.g(), color.b(), 255];
                //buffer[idx..(idx+4)].copy_from_slice(&px);
                self.buffer[idx]   = color.r();
                self.buffer[idx+1] = color.g();
                self.buffer[idx+2] = color.b();
                self.buffer[idx+3] = 255u8;
            }
        }

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&self.buffer), self.width, self.height).unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

        return Ok(());
    }
}

impl OriginDimensions for CanvasDisplay {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}
