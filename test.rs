#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod vector {
    mod check {
        impl super::V2 {
            pub fn is_null(&self) -> bool {
                self.x() == 0.0 && self.y() == 0.0
            }
            pub fn is_horizontal(&self) -> bool {
                self.x() != 0.0 && self.y() == 0.0
            }
            pub fn is_vertical(&self) -> bool {
                self.x() == 0.0 && self.y() != 0.0
            }
        }
    }
    mod chain_op {
        use super::{
            angle::{Angle, Rad},
            V2,
        };
        impl V2 {
            pub fn add(&mut self, rhs: V2) -> &mut Self {
                self.0 += rhs.x();
                self.1 += rhs.y();
                self
            }
            pub fn sub(&mut self, rhs: V2) -> &mut Self {
                self.0 -= rhs.x();
                self.1 -= rhs.y();
                self
            }
            pub fn scale(&mut self, value: f32) -> &mut Self {
                self.0 *= value;
                self.1 *= value;
                self
            }
            pub fn normalize(&mut self) -> &mut Self {
                let mag = self.mag();
                if mag != 0.0 {
                    self.scale(1.0 / mag);
                }
                self
            }
            pub fn set_direction<U: Into<Rad>>(&mut self, angle: U) -> &mut Self {
                if !self.is_null() {
                    let mag = self.mag();
                    let mut angle: Rad = angle.into();
                    self.0 = angle.cos() * mag;
                    self.1 = angle.sin() * mag;
                }
                self
            }
            pub fn rotate(&mut self, rad: f32) -> &mut Self {
                if !self.is_null() {
                    self.set_direction(rad + self.dir().unwrap());
                }
                self
            }
            pub fn rotate_over(&mut self, origin: V2, rad: f64) -> &mut Self {
                self.sub(origin);
                self.rotate(rad);
                self.add(origin);
                self
            }
            pub fn point_towards(&mut self, mut target: Vec2) -> &mut Self {
                self.set_direction(target.sub(*self).dir())
            }
        }
    }
    mod angle {
        use std::{f32::consts::PI, ops::{AddAssign, Deref, DerefMut}};
        use super::V2;
        pub struct Rad(f32);
        pub struct Deg(f32);
        impl From<Deg> for Rad {
            fn from(value: Deg) -> Self {
                Self(value.0 / 180.0 * PI)
            }
        }
        impl From<Rad> for Deg {
            fn from(value: Rad) -> Self {
                Self(value.0 / PI * 180.0)
            }
        }
        impl From<V2> for Rad {
            fn from(v2: V2) -> Self {
                if !v2.is_null() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Null vector does not have a direction!"),
                        );
                    }
                }
                Self(v2.y().atan2(v2.x()))
            }
        }
        impl From<V2> for Deg {
            fn from(v2: V2) -> Self {
                if !v2.is_null() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Null vector does not have a direction!"),
                        );
                    }
                }
                Self(v2.y().atan2(v2.x()) / PI * 180.0)
            }
        }
        impl From<Rad> for V2 {
            fn from(angle: Rad) -> Self {
                Self(angle.value().cos(), angle.value().sin())
            }
        }
        impl From<Deg> for V2 {
            fn from(angle: Deg) -> Self {
                let angle: Rad = angle.into();
                Self(angle.value().cos(), angle.value().sin())
            }
        }
        impl Rad {
            pub fn value(&self) -> f32 {
                self.0
            }
            pub fn add<T: Into<Rad>>(&mut self, rhs: T) -> &mut Self {
                let rhs: Rad = rhs.into();
                self.0 += rhs.value();
                self
            }
            pub fn cos(&self) -> f32 {
                (/*ERROR*/)
            }
            pub fn sin(&self) -> f32 {
                (/*ERROR*/)
            }
        }
        impl Deg {
            pub fn value(&self) -> f32 {
                self.0
            }
            pub fn add<T: Into<Deg>>(&mut self, rhs: T) -> &mut Self {
                let rhs: Deg = rhs.into();
                self.0 += rhs.value();
                self
            }
            pub fn cos(&self) -> f32 {
                (/*ERROR*/)
            }
            pub fn sin(&self) -> f32 {
                (/*ERROR*/)
            }
        }
    }
    pub struct V2(f32, f32);
    #[automatically_derived]
    impl ::core::clone::Clone for V2 {
        #[inline]
        fn clone(&self) -> V2 {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for V2 {}
    #[automatically_derived]
    impl ::core::fmt::Debug for V2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "V2", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for V2 {
        #[inline]
        fn default() -> V2 {
            V2(::core::default::Default::default(), ::core::default::Default::default())
        }
    }
    impl V2 {
        pub fn x(&self) -> f32 {
            self.0
        }
        pub fn y(&self) -> f32 {
            self.1
        }
        pub fn mag(&self) -> f32 {
            (self.x().powi(2) + self.y().powi(2)).powf(0.5)
        }
        pub fn dir(&self) -> V2 {
            let mut unit = self.clone();
            unit.normalize();
            unit
        }
    }
}
mod window {
    use std::{ffi::OsString, iter::once, os::windows::prelude::OsStrExt, ptr::null};
    use windows_sys::Win32::{
        Foundation::HWND, UI::WindowsAndMessaging::{FindWindowW, PostMessageW},
    };
    pub use windows_sys::Win32::UI::Input::KeyboardAndMouse;
    use image::{DynamicImage, GenericImageView, RgbaImage};
    use win_screenshot::prelude::*;
    use crate::Vec2;
    pub struct Window(HWND, isize);
    impl Window {
        pub fn get(name: &str) -> Self {
            let window_name = OsString::from(name);
            let window_name: Vec<u16> = window_name
                .as_os_str()
                .encode_wide()
                .chain(once(0))
                .collect();
            let handle: HWND = unsafe { FindWindowW(null(), window_name.as_ptr()) };
            Window(handle, find_window(name).unwrap())
        }
        fn post_message(&self, msg: u32, wparam: u16, lparam: isize) {
            let res = unsafe { PostMessageW(self.0, msg, wparam.into(), lparam) };
            if res == 0 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Failed to post message to window."),
                    );
                };
            }
        }
        pub fn get_screen_shot(&self) -> DynamicImage {
            let buf = capture_window(self.1).unwrap();
            DynamicImage::ImageRgba8(
                RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap(),
            )
        }
        pub fn foreach(&self) {
            for pixel in self.get_screen_shot().pixels() {
                let color: u32 = unsafe { std::mem::transmute(pixel.2.0) };
                if color == 0xff19ac3c {
                    {
                        ::std::io::_print(format_args!("{0:02x}\n", color));
                    };
                }
            }
        }
    }
    const WM_LBUTTONDOWN: u32 = 0x0201;
    const WM_LBUTTONUP: u32 = 0x0202;
    impl Window {
        pub fn mouse_left_down(&self) {
            self.post_message(WM_LBUTTONDOWN, 0, 0)
        }
        pub fn mouse_left_up(&self) {
            self.post_message(WM_LBUTTONUP, 0, 0)
        }
        pub fn mouse_move(&self, coords: Vec2) {
            let lp: isize = coords.x as isize;
            let lp = lp << 16;
            let lp = lp + coords.y as isize;
            self.post_message(512, 0, lp)
        }
        pub fn mouse_left_click(&self, coords: Vec2) {
            self.mouse_move(coords);
            self.mouse_left_down();
            self.mouse_left_up();
        }
    }
    const KEY_DOWN: u32 = 256;
    const KEY_UP: u32 = 257;
    impl Window {
        pub fn key_down(&self, key: KeyboardAndMouse::VIRTUAL_KEY) {
            self.post_message(KEY_DOWN, key, 0)
        }
        pub fn key_up(&self, key: KeyboardAndMouse::VIRTUAL_KEY) {
            self.post_message(KEY_UP, key, 0)
        }
        pub fn key_press_long(&self, key: KeyboardAndMouse::VIRTUAL_KEY, duration: u64) {
            self.key_down(key);
            std::thread::sleep(std::time::Duration::from_millis(duration));
            self.key_up(key);
        }
        pub fn key_press(&self, key: KeyboardAndMouse::VIRTUAL_KEY) {
            self.key_press_long(key, 0);
        }
    }
}
pub use vector::Vec2;
pub use window::Window;
fn main() {
    let game_window = Window::get("WarUniverse");
    game_window.key_press(window::KeyboardAndMouse::VK_X);
    let screen_center = Vec2::new(1920.0 / 2.0, 1080.0 / 2.0);
    let mut coords = Vec2::new(-500.0, -500.0);
    game_window.mouse_move(coords + screen_center);
    game_window.mouse_left_down();
    let mut count = 0;
    while count < 500 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        coords.rotate_over(screen_center, -0.05);
        game_window.mouse_move(coords + screen_center);
        count += 1;
    }
    game_window.mouse_left_up();
    game_window.get_screen_shot();
}
