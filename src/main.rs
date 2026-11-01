use macroquad::prelude::*;

#[macroquad::main("play pong against an ai")]
async fn main() {
    loop {
        draw_rectangle(50.0, 50.0, 50.0, 50.0, WHITE);

        next_frame().await;
    }
}
