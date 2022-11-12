use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb888,
    geometry::{OriginDimensions, Size},
    draw_target::DrawTarget
};

use pixels::Pixels;
use pixels::SurfaceTexture;

pub struct Pixelbuffer {
    width: u32,
    height: u32,
    pixels: Pixels,
}

use winit::window::Window;

const BYTES_PER_PIXEL: usize = 4;

impl Pixelbuffer {
    pub fn new(window: Window) -> Pixelbuffer {
        let size = window.inner_size();
        let st = SurfaceTexture::new(size.width, size.height, &window);
        Self {
            width: size.width,
            height: size.height,
            pixels: Pixels::new(size.width, size.height, st).unwrap(),
        }
    }
}

impl DrawTarget for Pixelbuffer {
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mut pxs = self.pixels.get_frame();
        for Pixel(coord, color) in pixels.into_iter() {
            // constrain coordinate to canvas area
            let (x, y) = (coord.x, coord.y);
            if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
                continue;
            } else {
                // calculate the buffer index
                let idx: usize = ((x as usize)*BYTES_PER_PIXEL) + (y as usize)*((self.width as usize)*BYTES_PER_PIXEL);
                let px: [u8; 4] = [color.r(), color.g(), color.b(), 255];
                pxs[idx..(idx+4)].copy_from_slice(&px);
                // pxs[idx]   = color.r();
                // pxs[idx+1] = color.g();
                // pxs[idx+2] = color.b();
                // pxs[idx+3] = 255u8;
            }
        }

        match self.pixels.render() {
            Ok(_) => return Ok(()),
            Err(_) => return Err(()),
        };

    }
}

impl OriginDimensions for Pixelbuffer {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}