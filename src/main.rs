extern crate raylib;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let (mut rl, thread) = raylib::init()
        .size(width, height).title("Corrosion2D").build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let pressed_keys = rl.get_key_pressed();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        if let Some(pressed_key) = pressed_keys {
            d.draw_text(&format!("{:?}", pressed_key), 100, 12, 10, Color::BLACK);
        }
        //println!("{:?}", pressed_key);
    }
}
