use fastrand::*;
use macroquad::prelude::*;

// helper functions and vars
fn is_off_screen_vertically(pos: Vec2) -> bool {
    return pos.y < 0.0 || pos.y > screen_height();
}
fn is_off_screen_horizontally(pos: Vec2) -> bool {
    return pos.x < 0.0 || pos.x > screen_width();
}
fn are_two_things_colliding(
    thing1_pos: Vec2,
    thing2_pos: Vec2,
    thing1_dimen: Vec2,
    thing2_dimen: Vec2,
) -> bool {
    let thing1_rect = Rect::new(thing1_pos.x, thing1_pos.y, thing1_dimen.x, thing1_dimen.y);
    let thing2_rect = Rect::new(thing2_pos.x, thing2_pos.y, thing2_dimen.x, thing2_dimen.y);

    if thing1_rect.overlaps(&thing2_rect) {
        return true;
    } else {
        return false;
    }
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

    // test hardcoded enemy
    let mut hardcoded_enemy = Paddle {
        y: screen_width() / 2.0,
        speed: 200.0,
        colour: RED,
        length: 75.0,
    };

    let mut main_ball = Ball {
        position: vec2(screen_width() / 2.0, screen_height() / 2.0),
        speed: 500.0,
        direction: vec2(f32(), f32()),
        size: 15.0,
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
        if is_off_screen_vertically(main_ball.position) {
            main_ball.direction.y *= -1.0;
        }
        if is_off_screen_horizontally(main_ball.position) {
            main_ball.position = vec2(screen_width() / 2.0, screen_height() / 2.0);
        }

        if are_two_things_colliding(
            vec2(10.0, player.y),
            main_ball.position,
            vec2(10.0, player.length),
            vec2(main_ball.size, main_ball.size),
        ) {
            // bounce it
            main_ball.direction.x *= -1.0;
            main_ball.direction.y *= -1.0;
        }

        // make enemy perfect (for now)
        hardcoded_enemy.y = main_ball.position.y;
        if are_two_things_colliding(
            vec2(screen_width() - 20.0, hardcoded_enemy.y),
            main_ball.position,
            vec2(10.0, hardcoded_enemy.length),
            vec2(main_ball.size, main_ball.size),
        ) {
            // bounce it
            main_ball.direction.x *= -1.0;
            main_ball.direction.y *= -1.0;
        }

        // render stuff
        draw_rectangle(10.0, player.y, 10.0, player.length, player.colour);
        draw_rectangle(
            screen_width() - 20.0,
            hardcoded_enemy.y,
            10.0,
            hardcoded_enemy.length,
            hardcoded_enemy.colour,
        );
        draw_circle(
            main_ball.position.x,
            main_ball.position.y,
            main_ball.size,
            main_ball.colour,
        );

        next_frame().await;
    }
}
