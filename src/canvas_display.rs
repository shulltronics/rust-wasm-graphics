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

pub struct CanvasDisplay {
    pub width: u32,
    pub height: u32,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl CanvasDisplay {
    pub fn new(c: web_sys::HtmlCanvasElement) -> CanvasDisplay {
        Self {
            width: c.width(),
            height: c.height(),
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
        const BYTES_PER_PX: i32 = 4;
        let mut buffer: Vec<u8> = vec![0u8; (self.width * self.height * 4) as usize];
        for Pixel(coord, color) in pixels.into_iter() {
            // constrain coordinate to canvas area
            let (x, y) = (coord.x, coord.y);
            /*
            if x < 0 
                || x >= self.width as i32
                || y < 0
                || y >= self.height as i32
            {
                continue;
            } else {
            */
                // calculate the buffer index
                let idx: usize = ((x*4) + y*((self.width as i32)*BYTES_PER_PX)) as usize;
                let px: [u8; 4] = [color.r(), color.g(), color.b(), 255];
                buffer[idx..(idx+4)].copy_from_slice(&px);
            //}
        }

        /*
        for i in 0..(self.width * self.height) {
            let loc: usize = (i * 4).try_into().unwrap();
            buffer[loc..(loc+4)].copy_from_slice(&[255u8, 255u8, 0u8, 255u8]);
        }
        */

        // Write the pixels to the canvas through an imageData object
        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), self.width, self.height).unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

        return Ok(());
    }
}

impl OriginDimensions for CanvasDisplay {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}
