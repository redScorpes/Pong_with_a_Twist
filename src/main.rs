use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space_Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let screen_width = window_conf().window_width as f32;
    let screen_height = window_conf().window_height as f32;


    let mut paddle1_hight: f32 = 70.0;
    let mut paddle1_width: f32 = 15.0;
    let mut paddle1_x: f32 = 10.0;
    let mut paddle1_y: f32 = 100.0;
    let mut paddle1_speed: f32 = 200.0;

    let mut paddle2_hight: f32 = 70.0;
    let mut paddle2_width: f32 = 15.0;
    let mut paddle2_x: f32 = screen_width - 25.0;
    let mut paddle2_y: f32 = 100.0;
    let mut paddle2_speed: f32 = 180.0;

    let mut ball_pos: Vec2 = vec2(screen_width / 2.0, screen_height / 2.0);
    let mut ball_size: Vec2 = vec2(10.0, 10.0);
    let mut ball_speed: Vec2 = vec2(100.0, 100.0);
    let mut ball_color: Color = WHITE;

    let barrier_thickness: f32 = 5.0;
    let top_barrier = (0.0, 0.0, screen_width, barrier_thickness);
    let bottom_barrier = (0.0, screen_height - barrier_thickness, screen_width, barrier_thickness);
    let left_barrier = (0.0, 0.0, barrier_thickness, screen_height);
    let right_barrier = (screen_width - barrier_thickness, 0.0, barrier_thickness, screen_height);

    let mut lives_player = 3;
    let mut lives_ai = 3;

    let mut game_started = false;



    loop {
        let delta_time = get_frame_time();

        ball_pos += ball_speed * delta_time;

        if is_key_pressed(KeyCode::Space) {
            game_started = true;
        }

        if game_started {
            ball_pos += ball_speed * delta_time;

            if ball_pos.y <= barrier_thickness || ball_pos.y + ball_size.y >= screen_height - barrier_thickness {
                ball_speed.y = -ball_speed.y;
            }

            if ball_pos.x <= barrier_thickness || ball_pos.x + ball_size.x >= screen_width - barrier_thickness {
                game_started = false;
            }

            if ball_pos.x <= paddle1_x + paddle1_width && ball_pos.y + ball_size.y >= paddle1_y && ball_pos.y <= paddle1_y + paddle1_hight {
                let hit_pos = (ball_pos.y + ball_size.y / 2.0) - paddle1_y;
                let hit_ratio = (hit_pos / paddle1_hight) - 0.5;
                ball_speed.x = -ball_speed.x;
                ball_speed.y += hit_ratio * 300.0;
            }

            if ball_pos.x + ball_size.x >= paddle2_x && ball_pos.y + ball_size.y >= paddle2_y && ball_pos.y <= paddle2_y + paddle2_hight {
                let hit_pos = (ball_pos.y + ball_size.y / 2.0) - paddle2_y;
                let hit_ratio = (hit_pos / paddle2_hight) - 0.5;
                ball_speed.x = -ball_speed.x;
                ball_speed.y += hit_ratio * 300.0;
            }
        } else {
            ball_pos = vec2(paddle1_x + paddle1_width + 5.0, paddle1_y + paddle1_hight / 2.0 - 5.0);
        }

        ball_speed = ball_speed.normalize() * 200.0;

        clear_background(BLACK);

        if paddle1_y > 0.0 + 10.0 {
            if is_key_down(KeyCode::W) {
                paddle1_y -= paddle1_speed * delta_time;
            }
        };

        if paddle1_y < screen_height - paddle1_hight - 10.0 {
            if is_key_down(KeyCode::S) {
                paddle1_y += paddle1_speed * delta_time;
            }
        }

        if ball_pos.y < paddle2_y + paddle2_hight / 2.0 {
            paddle2_y -= paddle2_speed * delta_time;
        } else {
            paddle2_y += paddle2_speed * delta_time;
        }


        draw_rectangle(paddle1_x, paddle1_y , paddle1_width, paddle1_hight, WHITE);
        draw_rectangle(paddle2_x, paddle2_y , paddle2_width, paddle2_hight, WHITE);

        draw_rectangle(ball_pos.x, ball_pos.y, ball_size.x, ball_size.y, ball_color);

        draw_rectangle(top_barrier.0, top_barrier.1, top_barrier.2, top_barrier.3, WHITE);
        draw_rectangle(bottom_barrier.0, bottom_barrier.1, bottom_barrier.2, bottom_barrier.3, WHITE);
        //draw_rectangle(left_barrier.0, left_barrier.1, left_barrier.2, left_barrier.3, WHITE);
        //draw_rectangle(right_barrier.0, right_barrier.1, right_barrier.2, right_barrier.3, WHITE);

        draw_text(&format!("Lives: {}", lives_player), 20.0, 100.0, 100.0, WHITE);
        draw_text(&format!("Lives: {}", lives_ai), screen_width - 50.0, 100.0, 100.0, WHITE);

        next_frame().await
    }
}
