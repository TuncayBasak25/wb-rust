use std::{ffi::OsString, iter::once, os::windows::prelude::OsStrExt, ptr::null};
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{FindWindowW, PostMessageW},
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
            panic!("Failed to post message to window.");
        }
    }

    pub fn get_screen_shot(&self) -> DynamicImage {
        let buf = capture_window(self.1).unwrap();
        
        DynamicImage::ImageRgba8(RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap())
    }

    pub fn foreach(&self) {
        for pixel in self.get_screen_shot().pixels() {
            let color: u32 = unsafe {std::mem::transmute(pixel.2.0)};

            if color == 0xff19ac3c {
                print!("{:02x}\n", color);
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