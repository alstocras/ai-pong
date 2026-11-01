use macroquad::prelude::*;

// helper functions and vars
fn is_off_screen(pos: Vec2) -> bool {
    return pos.x < 0.0 || pos.x > screen_width() || pos.y < 0.0 || pos.y > screen_height();
}

struct Paddle {
    y: f32,
    speed: f32,
    colour: Color,
    length: f32,
}

struct Ball {
    position: Vec2,
    speed: f32,
    direction: Vec2,
    size: f32,
    colour: Color,
}

#[macroquad::main("play pong against an ai")]
async fn main() {
    // player paddle
    let mut player = Paddle {
        y: screen_width() / 2.0,
        speed: 200.0,
        colour: GOLD,
        length: 75.0,
    };

    let mut main_ball = Ball {
        position: vec2(screen_width() / 2.0, screen_height() / 2.0),
        speed: 150.0,
        direction: vec2(-1.0, 0.0),
        size: 20.0,
        colour: WHITE,
    };

    loop {
        // to ensure the movement is the same regardless of framerate
        let dt = get_frame_time();
        let is_too_high: bool = player.y < 0.0;
        let is_too_low: bool = player.y >= screen_height() - player.length;

        // movement input (support for both qwerty and engrammer (mine))
        if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
            if !is_too_high {
                player.y -= player.speed * dt;
            }
        }
        if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
            if !is_too_low {
                player.y += player.speed * dt;
            }
        }

        // move ball
        let velocity: Vec2 = main_ball.direction * main_ball.speed * dt;
        main_ball.position += velocity;
        if is_off_screen(main_ball.position) {
            main_ball.position = vec2(screen_width() / 2.0, screen_height() / 2.0);
        }

        // render stuff
        draw_rectangle(10.0, player.y, 10.0, player.length, player.colour);
        draw_circle(
            main_ball.position.x,
            main_ball.position.y,
            main_ball.size,
            main_ball.colour,
        );

        next_frame().await;
    }
}
