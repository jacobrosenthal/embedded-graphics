//! # Example: Analog Clock
//!
//! ![Screenshot of clock example]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfwAAAH9AQMAAADFwFz1AAAABlBMVEUAAAD///+l2Z/dAAAFM0lEQVR42u3dYWrjMBAF4Afz1zBX8QEEc/WBvdAcQKBNussK3EbVpDNSt+SRYMWWPrBsybQJCK+88sp/EWm32BagN+/EcoBuLRUE8K2wA6B7c9yBN2I9QK3+2eItrS0HWv0L9c9LgV6f+p6lADdcAWq6AuiVO9DRhYBYty571wBc8RGApquApkM4H+A6lvOBpkM6H+A6tvOBpkM8H6A61BcAosMetnSA6olBzpYOsI0B0Wyg4cAgSjUZIPsEgCQDoqCRD7DlAvX2Hlyo414lFSD7HJAsoI/lc3An9xGdBFRMpCYCZDOAJAKsMwBrHiCYCVka0I+MI0GAvw97vSxAMBeyLKBiMjUJIJsFJAjwX8VeMwcQzIYsB6iYTk0ByOYBCQL8V7HXzQAE8yHLAAocSQHMA5QEgNQDcALAPkDjAYEnZPFAhSs1HjAfUMIBUh/A4QB7AY0GBL6QRQMFzoQD5gVKMEBuQKIB9QIcDDC8IY0FxA9YLFDgTjBQ/UCNBcwPlFCA1A9wKMBPAE1DgeI/AwoFxN+LJ1kkUHD655NQoLrntAOokYAB6h9NgQAZnohEAvoMwFHA+9HcHuUCaCCAZwAKBOQ5wOKAcgGU7q/WLoULgB8FXMZSlVqkisq1cAHKDwJIrzdSlXZvdysU6YUrwD8ZqHR/3VKpKeRvgVoawHgOIE0DfrX7S6X35q/bpkoaIM8CFgWUy4G3VnegSr21fSt8BOAnA2+75O9cet9WQG7lNMCeBUoW0B7kBwOkzwIcB/iSD+h2oKwGnv8XMWkSgHM7wNsB2Fqg4OlzsDQAuh0o2wHS3QDO7QCvBGxw7JOUTODYDpDuBlC2A6S7AZzbAd4OwBYB/XL5xxMnA6S7AZTtAG0HYNsB3g7AWDcDR6ubAfoGQFsAtGbPAdLal4Fv0gcjAK1+EfgGN9IXgcK6GVBsBhi7gXM7oDdjK8DYDZxrANioD8dASQYYO4F/bY+NAOlu4MBu4NwNkG4FekvKBUZ/WtkEYJkAYTdwbAesz6x7AMJu4FgJ8INDnwOkeQBhN1B68dwD6FKAdNCHQ4CRBpTtgO4G+DKws4H3T9dzEihpgO4GGKuBMujDIWBZgF6G9nKAsRyQQR+OAAoD+DJXTwOaAxxYCPTCaBLVwYyWAZDuBg5sAGDosXmgpACELUAv4XB8fWwpgO0B5NKHcwAFAtz70ANoIPCvaA6AA4E+O3q+QWfEAf2CeAAJBPAUUCKBCqgPOIAaCRQwfAApLBIQnHD+iOCkUIBJvQCThgIVXgAtFCD1A4xIYDyaPtZLLFD9QB0CbqD4AYsFxA1QMMB+QGOB/nH26coIBswLSDAAN1CigeIFLBoQJ0DhAKvv6coaDZAXQDQA8wElHqg+oMYD4gLI4gF2/daTNR4gH4B4AOYBSgZQPIBlAOIAKALw3En6vm4GQDYPCDIA1Hmg5gAyDZDlAKyzAGsOQDb7fBfkAKizQM0CZBIgywJY5wDWLIBsDhBkAZCp5ztZHsD6ALZxLT/g7UU9LicaALiGE4EulfIAUQD6wVXUfj3ZMgGy2xuPAep9mASgfg7UXEAUxwCAgi0XIMM5Ak4IcgE0nKNb+aSaDbDpaDgfotkAVR1NaUdDBOBfWkj7GeYDVEdAQz6Apo+frlxTgUGls+OpwONaZ7dXAFzfA51eAfR6HehyCOBfdu/o8BIAYleg710DUNP3ADesAnrlDnR0EYBWL0Dfkw90oQPU24cA/uVYqTV8Ffj/1rT9Buv6foO1jb/D+s6vvPKKI78B89G0YRkxkl8AAAAASUVORK5CYII=)
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.

use embedded_graphics::prelude::*;

use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};

//todo test with micromath to see if its good enough
// use micromath::F32Ext;

/// The width and height of the display
const DISP_SIZE_X: i32 = 160;
const DISP_SIZE_Y: i32 = 128;

//https://github.com/s-macke/VoxelSpace
fn voxelspace(
    p: Point,
    phi: f32,
    height: u32,
    horizon: u32,
    scale_height: u32,
    distance: u32,
    screen_width: i32,
    screen_height: i32,
    heightmap: &[u8],
    colormap: &[u8],
    mapwidth: i32,
    display: &mut SimulatorDisplay<Rgb565>,
) {
    // precalculate viewing angle parameters
    // let sinphi = phi.sin();
    // let cosphi = phi.cos();

    // Draw from back to the front (high z coordinate to low z coordinate)
    for z in (1..distance).rev() {
        // // Find line on map. This calculation corresponds to a field of view of 90°
        // let z = z as f32;
        // let mut pleft = Point::new(
        //     (-cosphi * z - sinphi * z) as i32 + p.x,
        //     (sinphi * z - cosphi * z) as i32 + p.y,
        // );

        // let pright = Point::new(
        //     (cosphi * z - sinphi * z) as i32 + p.x,
        //     (-sinphi * z - cosphi * z) as i32 + p.y,
        // );

        // // segment the line
        // let dx = (pright.x - pleft.x) / screen_width;
        // let dy = (pright.y - pleft.y) / screen_width;

        // Raster line and draw a vertical line for each segment
        for x in 0..screen_width {
            // let index = (pleft.y * mapwidth + pleft.x) as u32;

            // //this as usize panics
            // let height_val = heightmap[index as usize] as u32;
            // //todo is z as u16 ok?
            // let height_on_screen =
            //     ((height - height_val) / z as u32 * scale_height + horizon) as i32;

            //what was that x+1024 shit?
            (70..screen_height)
                .map(|y| {
                    //blah.. brittle.. but image library doesnt have x,y access
                    // *2 for u8 to u16
                    let index = ((y * mapwidth + x) * 2) as usize;
                    let color = u16::from_le_bytes([colormap[index], colormap[index + 1]]);
                    Pixel(Point::new(x as i32, y as i32), RawU16::new(color).into())
                })
                .draw(display);

            // pleft.x += dx;
            // pleft.y += dy;
        }
    }
}

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(128, 128));

    let mut window = WindowBuilder::new(&display)
        .title("Voxelspace")
        .scale(2)
        .build();

    //prepared into a columnar u8 via the build.rs
    let heightmap = include_bytes!("D7.raw");
    //prepared into a columnar LE u16 rgb565 via the build.rs
    let colormap = include_bytes!("C7W.raw");

    'running: loop {
        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'running;
            }
        }

        //hopefully this isnt pixel by pixel? or it is going to halve performance
        Rectangle::new(Point::new(0, 0), Point::new(DISP_SIZE_X, DISP_SIZE_Y))
            .into_styled(PrimitiveStyle::with_fill(RgbColor::BLUE))
            .draw(&mut display);

        voxelspace(
            Point::new(0, 0),
            0.0,         //phi
            50,          //height
            120,         //horizon
            120,         //scale_height
            300,         //distance
            DISP_SIZE_X, //screen_width
            DISP_SIZE_Y, //screen_height
            heightmap,
            colormap,
            150, //mapwidth
            &mut display,
        );

        window.update(&display);
    }
}
