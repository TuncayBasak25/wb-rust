

mod vector;
mod window;

pub use vector::Vec2;
pub use window::Window;




fn main() {
    let game_window = Window::get("WarUniverse");

    game_window.key_press(window::KeyboardAndMouse::VK_X);
    
    let screen_center = Vec2::new(1920.0/2.0, 1080.0/2.0);
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