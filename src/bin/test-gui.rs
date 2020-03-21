use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 1024)
        .title("Good Morning, Dr. Chandra. I am a HAL 9000 computer.")
        .build();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);        
        d.clear_background(Color::WHITE);
        d.draw_text("The AE35 Unit will completely fail in 72 hours.", 12, 12, 20, Color::BLUE);
    }
}

