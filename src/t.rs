use image::{DynamicImage, GenericImageView, RgbaImage};
use win_screenshot::prelude::*;

struct Window {
    handle: isize,
}

impl Window {

}


fn main() {
    let window = Window::new("WarUniverse");
    let img = window.get_screen_shot();
    let pixels = img.pixels();

    for pixel in pixels {
        let color: u32 = unsafe {std::mem::transmute(pixel.2.0)};

        if color == 0xff19ac3c {
            print!("{:02x}\n", color);
        }
    }

}
 