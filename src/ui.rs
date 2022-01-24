use embedded_graphics::{
    prelude::*,
    text::Text,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
};

pub fn draw_text() {
    let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    let text = Text::new("hello embedded graphics through Wasm!", Point::new(0, 0), style);
}
