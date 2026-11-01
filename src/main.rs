use catppuccin::PALETTE;
use macroquad::prelude::*;

#[macroquad::main("play pong against an ai")]
async fn main() {
    // start player at centre
    let mut player_paddle_y: f32 = screen_height() / 2.0;
    const PADDLE_LENGTH: f32 = 75.0;
    const PADDLE_SPEED: f32 = 200.0;

    loop {
        // to ensure the movement is the same regardless of framerate
        let dt = get_frame_time();
        let is_too_high: bool = player_paddle_y < 0.0;
        let is_too_low: bool = player_paddle_y >= screen_height() - PADDLE_LENGTH;

        // movement input (support for both qwerty and engrammer (mine))
        if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
            if !is_too_high {
                player_paddle_y -= PADDLE_SPEED * dt;
            }
        }
        if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
            if !is_too_low {
                player_paddle_y += PADDLE_SPEED * dt;
            }
        }

        // render stuff
        draw_rectangle(10.0, player_paddle_y, 10.0, PADDLE_LENGTH, WHITE);

        next_frame().await;
    }
}
