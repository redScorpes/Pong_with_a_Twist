use std::time::SystemTime;
use macroquad::prelude::*;
use macroquad::rand::{self, srand};

fn window_conf() -> Conf {
    Conf {
        window_title: "Space_Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_lives_player(lives_player: i32, pong_heart: &Texture2D) {
    if lives_player == 3 {
        draw_texture(pong_heart, 10.0, 10.0, WHITE);
        draw_texture(pong_heart, 30.0, 10.0, WHITE);
        draw_texture(pong_heart, 50.0, 10.0, WHITE);
    } else if lives_player == 2 {
        draw_texture(pong_heart, 10.0, 10.0, WHITE);
        draw_texture(pong_heart, 30.0, 10.0, WHITE);
    } else if lives_player == 1 {
        draw_texture(pong_heart, 10.0, 10.0, WHITE);
    }
}

fn draw_lives_ai(lives_ai: i32, pong_heart: &Texture2D, screen_width: f32) {
    if lives_ai == 3 {
        draw_texture(pong_heart, screen_width - 60.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 40.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 20.0, 10.0, WHITE);
    } else if lives_ai == 2 {
        draw_texture(pong_heart, screen_width - 40.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 20.0, 10.0, WHITE);
    } else if lives_ai == 1 {
        draw_texture(pong_heart, screen_width - 20.0, 10.0, WHITE);
    }
}

fn draw_powerup(powerup_shield: &Texture2D, powerup_speed: &Texture2D, powerup_size: &Texture2D, powerup_multi: &Texture2D, powerup_num: &i32, powerup_location: &Vec2, mut powerup_collected: &bool) {
    if !powerup_collected {
        if powerup_num == &0 {
            draw_texture(powerup_shield, powerup_location.x, powerup_location.y, WHITE);
        } else if powerup_num == &1 {
            draw_texture(powerup_speed, powerup_location.x, powerup_location.y, WHITE);
        } else if powerup_num == &2 {
            draw_texture(powerup_size, powerup_location.x, powerup_location.y, WHITE);
        } else if powerup_num == &3 {
            draw_texture(powerup_multi, powerup_location.x, powerup_location.y, WHITE);
        }
    }
}

fn rects_collide(rect1_pos: Vec2, rect1_size: Vec2, rect2_pos: Vec2, rect2_size: Vec2) -> bool {
    rect1_pos.x < rect2_pos.x + rect2_size.x &&
        rect1_pos.x + rect1_size.x > rect2_pos.x &&
        rect1_pos.y < rect2_pos.y + rect2_size.y &&
        rect1_pos.y + rect1_size.y > rect2_pos.y
}


#[macroquad::main(window_conf)]
async fn main() {

    let pong_heart = load_texture("assets/Pong_Heart.png").await.unwrap();

    let screen_width = window_conf().window_width as f32;
    let screen_height = window_conf().window_height as f32;


    let mut paddle1_hight: f32 = 70.0;
    let mut paddle1_width: f32 = 15.0;
    let mut paddle1_x: f32 = 10.0;
    let mut paddle1_y: f32 = 100.0;
    let mut paddle1_speed: f32 = 200.0;
    let mut paddle1_colour: Color = WHITE;

    let mut paddle2_hight: f32 = 70.0;
    let mut paddle2_width: f32 = 15.0;
    let mut paddle2_x: f32 = screen_width - 25.0;
    let mut paddle2_y: f32 = 100.0;
    let mut paddle2_speed: f32 = 300.0;

    let mut ball_pos: Vec2 = vec2(screen_width / 2.0, screen_height / 2.0);
    let mut ball_size: Vec2 = vec2(10.0, 10.0);
    let mut ball_direction: Vec2 = vec2(0.5, 0.5);
    let default_ball_speed = 600.0;
    let mut ball_speed: f32 = default_ball_speed.clone();
    let mut ball_color: Color = WHITE;

    let barrier_thickness: f32 = 5.0;
    let top_barrier = (0.0, 0.0, screen_width, barrier_thickness);
    let bottom_barrier = (0.0, screen_height - barrier_thickness, screen_width, barrier_thickness);
    let shield = (0.0, 5.0, barrier_thickness, screen_height - 10.0);

    let mut lives_player = 3;
    let mut lives_ai = 3;

    let mut game_started = false;
    let mut game_over = false;

    let powerup_shield = load_texture("assets/Pong_PowerUp_Shield.png").await.unwrap();
    let powerup_speed = load_texture("assets/Pong_PowerUp_Speed.png").await.unwrap();
    let powerup_size = load_texture("assets/Pong_PowerUp_Size.png").await.unwrap();
    let powerup_multi = load_texture("assets/Pong_PowerUp_Mult.png").await.unwrap();

    let mut powerup_collected = false;
    let mut powerup_active = false;

    srand(miniquad::date::now() as u64);
    let mut powerup_num = rand::gen_range(0, 4);
    let mut powerup_location = vec2(screen_width / 2.0, rand::gen_range(100.0, screen_height - 100.0));

    let mut shield_time = 0.0;
    let mut speed_time = 0.0;
    let mut size_time = 0.0;
    let mut multi_time = 0.0;

    let mut paddle1_hit = false;

    loop {
        let delta_time = get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            game_started = true;
        }

        if game_started {

            ball_pos += ball_direction * delta_time * ball_speed;

            if powerup_active {
                powerup_location.x -= 100.0 * delta_time;
            }

            if shield_time > 0.0 {
                shield_time -= delta_time;
            } else {
                paddle1_width = 15.0;
            }
            if speed_time > 0.0 {
                ball_color = SKYBLUE;
                if paddle1_hit {
                    ball_color = PURPLE;
                    ball_speed = 1000.0;
                    speed_time -= delta_time;
                }
            } else {
                ball_speed = default_ball_speed.clone();
                paddle1_hit = false;
                ball_color = WHITE;
            }
            if size_time > 0.0 {
                size_time -= delta_time;
                paddle1_colour = GREEN;
            } else {
                paddle1_hight = 70.0;
                paddle1_colour = WHITE;
            }
            if multi_time > 0.0 {
                ball_color = RED;
                multi_time -= delta_time;
            } else {

            }

            if rects_collide(vec2(top_barrier.0, top_barrier.1), vec2(top_barrier.2, top_barrier.3), ball_pos, ball_size) {
                ball_direction.y = -ball_direction.y;
                ball_pos.y = top_barrier.1 + top_barrier.3;
            }
            if rects_collide(vec2(bottom_barrier.0, bottom_barrier.1), vec2(bottom_barrier.2, bottom_barrier.3), ball_pos, ball_size) {
                ball_direction.y = -ball_direction.y;
                ball_pos.y = bottom_barrier.1 - ball_size.y;
            }

            if shield_time > 0.0 {
                if rects_collide(vec2(shield.0, shield.1), vec2(shield.2, shield.3), ball_pos, ball_size) {
                    ball_direction.x = -ball_direction.x;
                }
            }

            if ball_pos.x <= 0.0 || ball_pos.x + ball_size.x >= screen_width {
                if ball_pos.x <= 0.0 {
                    lives_player -= 1;
                } else {
                    lives_ai -= 1;
                }
                ball_color = WHITE;
                paddle1_colour = WHITE;
                paddle1_hight = 70.0;
                shield_time = 0.0;
                speed_time = 0.0;
                size_time = 0.0;
                multi_time = 0.0;
                game_started = false;
                ball_direction = vec2(0.5, 0.5);
                ball_speed = default_ball_speed.clone();
                powerup_num = rand::gen_range(0, 4);
                powerup_active = false;
                powerup_collected = false;
            }


            if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), ball_pos, ball_size) {
                if ball_pos.y < paddle1_y || ball_pos.y > paddle1_y + paddle1_hight {
                    ball_direction.y = -ball_direction.y;
                } else {
                    ball_direction.x = -ball_direction.x;
                    ball_pos.x = paddle1_x + paddle1_width;
                    paddle1_hit = true;
                }
            }
            if rects_collide(vec2(paddle2_x, paddle2_y), vec2(paddle2_width / 4.0, paddle2_hight), ball_pos, ball_size) {
                if ball_pos.y < paddle2_y || ball_pos.y > paddle2_y + paddle2_hight {
                    ball_direction.y = -ball_direction.y;
                } else {
                    ball_direction.x = -ball_direction.x;
                    ball_pos.x = paddle2_x - ball_size.x;
                }
                if !powerup_collected && !powerup_active {
                    powerup_location = vec2(paddle2_x - 20.0, paddle2_y + paddle2_hight / 2.0);
                }
                powerup_active = true;
            }

            if powerup_location.x < 0.0 {
                powerup_active = false;
                powerup_collected = false;
                powerup_num = rand::gen_range(0, 4);
            }

            if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), powerup_location, vec2(50.0, 50.0)) && powerup_active {
                powerup_collected = true;
                paddle1_hit = false;


                if powerup_num == 0 {
                    shield_time = 10.0;
                } else if powerup_num == 1 {
                    speed_time = 10.0;
                } else if powerup_num == 2 {
                    size_time = 10.0;
                    paddle1_hight += 50.0;
                    paddle1_y -= 25.0;
                } else if powerup_num == 3 {
                    multi_time = 10.0;
                }



                powerup_collected = false;
                powerup_num = rand::gen_range(0, 4);
                powerup_location = vec2(screen_width / 2.0, rand::gen_range(0.0, screen_height - 20.0));
            }
        } else {
            ball_pos = vec2(paddle1_x + paddle1_width + 5.0, paddle1_y + paddle1_hight / 2.0 - 5.0);
        }

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

        if ball_pos.y < paddle2_y + paddle2_hight / 2.0 && paddle2_y > 10.0 {
            paddle2_y -= paddle2_speed * delta_time;
        } else if paddle2_y < screen_height - paddle2_hight - 10.0 {
            paddle2_y += paddle2_speed * delta_time;
        }


        draw_rectangle(paddle1_x, paddle1_y , paddle1_width, paddle1_hight, paddle1_colour);
        draw_rectangle(paddle2_x, paddle2_y , paddle2_width, paddle2_hight, WHITE);

        draw_rectangle(ball_pos.x, ball_pos.y, ball_size.x, ball_size.y, ball_color);

        draw_rectangle(top_barrier.0, top_barrier.1, top_barrier.2, top_barrier.3, WHITE);
        draw_rectangle(bottom_barrier.0, bottom_barrier.1, bottom_barrier.2, bottom_barrier.3, WHITE);

        if shield_time > 0.0 {
            draw_rectangle(shield.0, shield.1, shield.2, shield.3, BLUE);
        }

        draw_lives_player(lives_player, &pong_heart);
        draw_lives_ai(lives_ai, &pong_heart, screen_width);

        if powerup_active {
            draw_powerup(&powerup_shield, &powerup_speed, &powerup_size, &powerup_multi, &powerup_num, &powerup_location, &powerup_collected);
        }

        if lives_player == 0 {
                draw_text("Game Over", screen_width / 2.0 - 100.0, screen_height / 2.0, 40.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    lives_player = 3;
                    lives_ai = 3;
                    game_started = false;
                }
        } else if lives_ai == 0 {
                draw_text("You Win", screen_width / 2.0 - 100.0, screen_height / 2.0, 40.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    lives_player = 3;
                    lives_ai = 3;
                    game_started = false;
                }

        } else if !game_started {
            draw_text("Press Space to Start", screen_width / 2.0 - 100.0, screen_height / 2.0, 20.0, WHITE);
        }



        next_frame().await
    }
}
