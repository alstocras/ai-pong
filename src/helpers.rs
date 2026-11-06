// helper functions and vars
pub fn is_off_screen_vertically(pos: vec2) -> bool {
    return pos.y < 0.0 || pos.y > screen_height();
}
pub fn is_off_screen_horizontally(pos: vec2) -> bool {
    return pos.x < 0.0 || pos.x > screen_width();
}
pub fn are_two_things_colliding(
    thing1_pos: vec2,
    thing2_pos: vec2,
    thing1_dimen: vec2,
    thing2_dimen: vec2,
) -> bool {
    let thing1_rect = rect::new(thing1_pos.x, thing1_pos.y, thing1_dimen.x, thing1_dimen.y);
    let thing2_rect = rect::new(thing2_pos.x, thing2_pos.y, thing2_dimen.x, thing2_dimen.y);

    if thing1_rect.overlaps(&thing2_rect) {
        return true;
    } else {
        return false;
    }
}
pub fn perform_paddle_ball_collision(paddle: &mut paddle, ball: &mut ball) {
    if are_two_things_colliding(
        vec2(paddle.x, paddle.y),
        ball.position,
        vec2(paddle.width, paddle.length),
        vec2(ball.size, ball.size),
    ) {
        // bounce it
        ball.direction.x *= -1.0;
        ball.direction.y *= -1.0;
        ball.direction.y += paddle.direction.y + 0.05;
        ball.direction.x += paddle.direction.x + 0.05;
        ball.speed += paddle.speed / 10.0;
    }
}
pub fn player_gets_the_point(ball: &ball) -> bool {
    if ball.position.x >= screen_width() {
        return true;
    } else {
        return false;
    }
}
pub fn enemy_gets_the_point(ball: &ball) -> bool {
    if ball.position.x <= 0.0 {
        return true;
    } else {
        return false;
    }
}
