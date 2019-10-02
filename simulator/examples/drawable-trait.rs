//! Demonstrate the chaining abilities of embedded graphics iterators

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawing;
use embedded_graphics_simulator::{BinaryColorTheme, BinaryDisplay, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Experiment")
        .theme(BinaryColorTheme::OledBlue)
        .build_binary();

    Rectangle::new(Point::new(10, 10), Point::new(20, 20))
        .stroke_color(Some(BinaryColor::On))
        .into_iter()
        // .chain(
        //     Rectangle::new(Point::new(10, 10), Point::new(20, 20))
        //         .stroke_color(Some(BinaryColor::On)),
        // )
        .draw_ext(&mut display);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
