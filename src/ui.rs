use embedded_graphics::{
    prelude::*,
    text::Text,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::Rgb888,
};

pub fn draw_text() -> Text<'static, MonoTextStyle<'static, Rgb888>> {
    // Now construct the graphics
    // let circle = {
    //     let style = PrimitiveStyleBuilder::new()
    //         .stroke_color(Rgb888::RED)
    //         .stroke_width(1)
    //         .fill_color(Rgb888::GREEN)
    //         .build();
    //     Circle::new(Point::new(10, 10), 50)
    //         .into_styled(style)
    // };

    // let text = {
    //     let style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);
    //     Text::new("hello Web Ass!", Point::new(0, 15), style)
    // };

    let style = MonoTextStyle::new(&FONT_6X9, Rgb888::RED);
    let text = Text::new("hello embedded graphics through Wasm!", Point::new(30, 30), style);
    return text;
}
