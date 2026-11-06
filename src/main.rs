mod ai;
mod helpers;
mod types;

use ai::*;
use fastrand::*;
use helpers::*;
use macroquad::prelude::*;
use types::*;

#[macroquad::main("play pong against an ai")]
async fn main() {
    let game_state = GameState::Menu;

    // player paddle
    let mut player = Paddle {
        y: screen_height() / 2.0,
        x: 10.0,
        speed: 500.0,
        colour: GOLD,
        length: 75.0,
        width: 10.0,
        direction: vec2(0.0, 0.0),
        points: 0,
    };

    // test hardcoded enemy
    let mut hardcoded_enemy = Paddle {
        y: screen_height() / 2.0,
        x: screen_width() - 10.0,
        speed: 200.0,
        colour: RED,
        length: 75.0,
        width: 10.0,
        direction: vec2(0.0, 0.0),
        points: 0,
    };

    let mut main_ball = Ball {
        position: vec2(screen_width() / 2.0, screen_height() / 2.0),
        speed: 500.0,
        direction: vec2(f32(), f32()),
        size: 15.0,
        colour: WHITE,
    };

    loop {
        match game_state {
            GameState::Game => {
                // reset direction
                player.direction = vec2(0.0, 0.0);
                hardcoded_enemy.direction = vec2(0.0, 0.0);

                // to ensure the movement is the same regardless of framerate
                let dt = get_frame_time();
                let is_too_high: bool = player.y < 0.0;
                let is_too_low: bool = player.y >= screen_height() - player.length;

                // movement input (support for both qwerty and engrammer (mine))
                if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
                    if !is_too_high {
                        player.y -= player.speed * dt;
                        player.direction = vec2(0.0, -1.0);
                    }
                }
                if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
                    if !is_too_low {
                        player.y += player.speed * dt;
                        player.direction = vec2(0.0, 1.0);
                    }
                }

                // test my nn functions
                let main_neural = NeuralNetwork::init_nn();
                main_neural.run_through_net(&[6.0, 7.0]);

                // move ball
                let velocity: Vec2 = main_ball.direction * main_ball.speed * dt;
                main_ball.position += velocity;
                // calc points
                if player_gets_the_point(&main_ball) {
                    player.points += 1;
                }
                if enemy_gets_the_point(&main_ball) {
                    hardcoded_enemy.points += 1;
                }
                if is_off_screen_vertically(main_ball.position) {
                    main_ball.direction.y *= -1.0;
                }
                if is_off_screen_horizontally(main_ball.position) {
                    main_ball.position = vec2(screen_width() / 2.0, screen_height() / 2.0);
                }

                // update enemy x (to stay in the correct spot even if window is resized)
                hardcoded_enemy.x = screen_width() - hardcoded_enemy.width - 10.0;

                // make enemy OP (for now)
                hardcoded_enemy.y = main_ball.position.y - hardcoded_enemy.length / 2.0;

                perform_paddle_ball_collision(&mut player, &mut main_ball);
                perform_paddle_ball_collision(&mut hardcoded_enemy, &mut main_ball);

                // render stuff
                draw_rectangle(
                    player.x,
                    player.y,
                    player.width,
                    player.length,
                    player.colour,
                );
                draw_rectangle(
                    hardcoded_enemy.x,
                    hardcoded_enemy.y,
                    hardcoded_enemy.width,
                    hardcoded_enemy.length,
                    hardcoded_enemy.colour,
                );
                draw_circle(
                    main_ball.position.x,
                    main_ball.position.y,
                    main_ball.size,
                    main_ball.colour,
                );

                // points counter
                let point_text = format!("{}:{}", player.points, hardcoded_enemy.points);
                let point_text_dimen = measure_text(&point_text, None, 100, 1.0);

                draw_text(
                    &point_text,
                    screen_width() / 2.0 - point_text_dimen.width / 2.0,
                    point_text_dimen.height + 20.0,
                    100.0,
                    GREEN,
                );
            }
        }

        next_frame().await;
    }
}
