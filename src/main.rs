use raylib::prelude::*;

fn main() {
    let (mut handle, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !handle.window_should_close() {
        let mut draw = handle.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);
        draw.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
