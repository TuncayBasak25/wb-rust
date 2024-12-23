use vector::{V2f, V2i};
use window::Window;

mod vector;
mod window;





fn main() {
    let game_window = Window::get("WarUniverse");

    game_window.key_press(window::KeyboardAndMouse::VK_X);
    
    let screen_center = V2i(1920, 1080) / 2;
    let mut coords = V2f(300.0, 0.0);

    let mut count = 0;
    while count < 500 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        coords.rotate_mut(-0.05);
        game_window.mouse_left_click(coords.add(screen_center.into()).into());

        count += 1;
    }

    game_window.mouse_left_up();

    game_window.get_screen_shot();
}