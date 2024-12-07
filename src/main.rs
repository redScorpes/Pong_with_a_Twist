use std::time::SystemTime;
use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound};
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

    let mut shield_colour: Color = WHITE;

    let mut lives_player = 3;
    let mut lives_ai = 3;

    let mut game_started = false;
    let mut game_over = false;

    let powerup_shield = load_texture("assets/Pong_PowerUp_Shield.png").await.unwrap();
    let powerup_speed = load_texture("assets/Pong_PowerUp_Speed.png").await.unwrap();
    let powerup_size = load_texture("assets/Pong_PowerUp_Size.png").await.unwrap();
    let powerup_multi = load_texture("assets/Pong_PowerUp_Mult.png").await.unwrap();
    let powerup_icon = load_texture("assets/Pong_PowerUp.png").await.unwrap();

    let wasd = load_texture("assets/Pong_wasd.png").await.unwrap();
    let space_bar = load_texture("assets/Pong_SpaceBar.png").await.unwrap();

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

    let mut second_ball = false;

    let mut ball2_pos = ball_pos.clone();
    let mut ball2_direction: Vec2 = vec2( -0.5,  -0.5);
    let mut ball2_size: Vec2 = ball_size.clone();
    let mut ball2_color: Color = WHITE;

    let pong_hit = load_sound("assets/sounds/pong_hit.wav").await.unwrap();
    let goal_hit = load_sound("assets/sounds/goal_hit.wav").await.unwrap();
    let powerup_collect = load_sound("assets/sounds/powerup_collect.wav").await.unwrap();
    let countdown = load_sound("assets/sounds/countdown.wav").await.unwrap();

    let mut countdown_time = 0.0;

    let mut first_start = true;


    loop {

        let delta_time = get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            game_started = true;
            first_start = false;
        }

        if game_started {

            if second_ball {
                ball2_pos += ball2_direction * delta_time * ball_speed;
            }

            ball_pos += ball_direction * delta_time * ball_speed;

            if powerup_active {
                powerup_location.x -= 100.0 * delta_time;
            }

            if shield_time > 0.0 {
                shield_time -= delta_time;
                if shield_time <= 2.0 {
                    countdown_time -= delta_time;
                    if countdown_time <= 0.0 {
                        play_sound_once(&countdown);
                        countdown_time = 0.2;
                    }
                    if (shield_time * 10.0) as i32 % 2 == 0 {
                        shield_colour = BLUE;
                    } else {
                        shield_colour = WHITE;
                    }
                } else {
                    shield_colour = BLUE;
                }
            } else {
                shield_colour = WHITE;
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
                if size_time <= 2.0 {
                    countdown_time -= delta_time;
                    if countdown_time <= 0.0 {
                        play_sound_once(&countdown);
                        countdown_time = 0.2;
                    }
                    if (size_time * 10.0) as i32 % 2 == 0 {
                        paddle1_colour = GREEN;
                    } else {
                        paddle1_colour = WHITE;
                    }
                } else {
                    paddle1_colour = GREEN;
                }
            } else {
                paddle1_hight = 70.0;
                paddle1_colour = WHITE;
            }

            if multi_time > 0.0 {
                multi_time -= delta_time;
                if multi_time <= 2.0 {
                    countdown_time -= delta_time;
                    if countdown_time <= 0.0 {
                        play_sound_once(&countdown);
                        countdown_time = 0.2;
                    }
                    if (multi_time * 10.0) as i32 % 2 == 0 {
                        ball2_color = WHITE;
                    } else {
                        ball2_color = BLACK;
                    }
                } else {
                    ball_color = WHITE;
                }
            } else {
                second_ball = false;
                ball2_color = WHITE;
            }

            if rects_collide(vec2(top_barrier.0, top_barrier.1), vec2(top_barrier.2, top_barrier.3), ball_pos, ball_size) {
                ball_direction.y = -ball_direction.y;
                ball_pos.y = top_barrier.1 + top_barrier.3;
                play_sound_once(&pong_hit);
            }

            if rects_collide(vec2(bottom_barrier.0, bottom_barrier.1), vec2(bottom_barrier.2, bottom_barrier.3), ball_pos, ball_size) {
                ball_direction.y = -ball_direction.y;
                ball_pos.y = bottom_barrier.1 - ball_size.y;
                play_sound_once(&pong_hit);
            }
            if second_ball {
                if rects_collide(vec2(top_barrier.0, top_barrier.1), vec2(top_barrier.2, top_barrier.3), ball2_pos, ball2_size) {
                    ball2_direction.y = -ball2_direction.y;
                    ball2_pos.y = top_barrier.1 + top_barrier.3;
                    play_sound_once(&pong_hit);
                } else if rects_collide(vec2(bottom_barrier.0, bottom_barrier.1), vec2(bottom_barrier.2, bottom_barrier.3), ball2_pos, ball2_size) {
                    ball2_direction.y = -ball2_direction.y;
                    ball2_pos.y = bottom_barrier.1 - ball2_size.y;
                    play_sound_once(&pong_hit);
                }
            }

            if shield_time > 0.0 {
                if rects_collide(vec2(shield.0, shield.1), vec2(shield.2, shield.3), ball_pos, ball_size) {
                    ball_direction.x = -ball_direction.x;
                    play_sound_once(&pong_hit);
                } else if second_ball && rects_collide(vec2(shield.0, shield.1), vec2(shield.2, shield.3), ball2_pos, ball2_size) {
                    ball2_direction.x = -ball2_direction.x;
                    play_sound_once(&pong_hit);
                }
            }

            if ball_pos.x <= 0.0 || ball_pos.x + ball_size.x >= screen_width || second_ball && ball2_pos.x <= 0.0 || second_ball && ball2_pos.x + ball2_size.x >= screen_width {
                play_sound_once(&goal_hit);
                if ball_pos.x <= 0.0 {
                    lives_player -= 1;
                } else {
                    lives_ai -= 1;
                }
                ball_color = WHITE;
                ball2_color = WHITE;
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
                second_ball = false;
            }

            if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), ball_pos, ball_size) {
                if ball_pos.y < paddle1_y || ball_pos.y > paddle1_y + paddle1_hight {
                    ball_direction.y = -ball_direction.y;
                    play_sound_once(&pong_hit);
                } else {
                    ball_direction.x = -ball_direction.x;
                    ball_pos.x = paddle1_x + paddle1_width;
                    paddle1_hit = true;
                    play_sound_once(&pong_hit);
                }
            }
            if second_ball {
                if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), ball2_pos, ball2_size) {
                    if ball2_pos.y < paddle1_y || ball2_pos.y > paddle1_y + paddle1_hight {
                        ball2_direction.y = -ball2_direction.y;
                        play_sound_once(&pong_hit);
                    } else {
                        ball2_direction.x = -ball2_direction.x;
                        ball2_pos.x = paddle1_x + paddle1_width;
                        paddle1_hit = true;
                        play_sound_once(&pong_hit);
                    }
                }
            }
            if rects_collide(vec2(paddle2_x, paddle2_y), vec2(paddle2_width / 4.0, paddle2_hight), ball_pos, ball_size) {
                if ball_pos.y < paddle2_y || ball_pos.y > paddle2_y + paddle2_hight {
                    ball_direction.y = -ball_direction.y;
                    play_sound_once(&pong_hit);
                } else {
                    ball_direction.x = -ball_direction.x;
                    ball_pos.x = paddle2_x - ball_size.x;
                    play_sound_once(&pong_hit);
                }
                if !powerup_collected && !powerup_active {
                    powerup_location = vec2(paddle2_x - 20.0, paddle2_y + paddle2_hight / 2.0 - 25.0);
                }
                powerup_active = true;
            }
            if second_ball {
                if rects_collide(vec2(paddle2_x, paddle2_y), vec2(paddle2_width / 4.0, paddle2_hight), ball2_pos, ball2_size) {
                    if ball2_pos.y < paddle2_y || ball2_pos.y > paddle2_y + paddle2_hight {
                        ball2_direction.y = -ball2_direction.y;
                        play_sound_once(&pong_hit);
                    } else {
                        ball2_direction.x = -ball2_direction.x;
                        ball2_pos.x = paddle2_x - ball2_size.x;
                        play_sound_once(&pong_hit);
                    }
                    if !powerup_collected && !powerup_active {
                        powerup_location = vec2(paddle2_x - 20.0, paddle2_y + paddle2_hight / 2.0 - 25.0);
                        play_sound_once(&pong_hit);
                    }
                }
            }

            if powerup_location.x < 0.0 {
                powerup_active = false;
                powerup_collected = false;
                powerup_num = rand::gen_range(0, 4);
            }

            if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), powerup_location, vec2(50.0, 50.0)) && powerup_active {
                powerup_collected = true;
                paddle1_hit = false;
                play_sound_once(&powerup_collect);


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
                    second_ball = true;
                    ball2_pos = ball_pos.clone();
                    ball2_direction = vec2(-ball_direction.x, -ball_direction.y);
                }


                powerup_collected = false;
                powerup_active = false;
                powerup_num = rand::gen_range(0, 4);
                powerup_location = vec2(screen_width / 2.0, rand::gen_range(0.0, screen_height - 20.0));
            }
        } else {
            ball_pos = vec2(paddle1_x + paddle1_width + 5.0, paddle1_y + paddle1_hight / 2.0 - 5.0);
            ball2_pos = ball_pos.clone();
        }

        clear_background(BLACK);

        if is_key_down(KeyCode::Space) {
            paddle1_speed = 400.0;
        } else {
            paddle1_speed = 200.0;
        }

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

        if second_ball && ball2_pos.x > ball_pos.x {
            if ball2_pos.y < paddle2_y + paddle2_hight / 2.0 && paddle2_y > 10.0 {
                paddle2_y -= paddle2_speed * delta_time;
            } else if paddle2_y < screen_height - paddle2_hight - 10.0 {
                paddle2_y += paddle2_speed * delta_time;
            }
        } else {
            if ball_pos.y < paddle2_y + paddle2_hight / 2.0 && paddle2_y > 10.0 {
                paddle2_y -= paddle2_speed * delta_time;
            } else if paddle2_y < screen_height - paddle2_hight - 10.0 {
                paddle2_y += paddle2_speed * delta_time;
            }
        }


        draw_rectangle(paddle1_x, paddle1_y , paddle1_width, paddle1_hight, paddle1_colour);
        draw_rectangle(paddle2_x, paddle2_y , paddle2_width, paddle2_hight, WHITE);

        draw_rectangle(ball_pos.x, ball_pos.y, ball_size.x, ball_size.y, ball_color);

        if second_ball {
            draw_rectangle(ball2_pos.x, ball2_pos.y, ball_size.x, ball_size.y, ball2_color);
        }

        draw_rectangle(top_barrier.0, top_barrier.1, top_barrier.2, top_barrier.3, WHITE);
        draw_rectangle(bottom_barrier.0, bottom_barrier.1, bottom_barrier.2, bottom_barrier.3, WHITE);

        if shield_time > 0.0 {
            draw_rectangle(shield.0, shield.1, shield.2, shield.3, shield_colour);
        }

        draw_lives_player(lives_player, &pong_heart);
        draw_lives_ai(lives_ai, &pong_heart, screen_width);

        if powerup_active {
            draw_powerup(&powerup_shield, &powerup_speed, &powerup_size, &powerup_multi, &powerup_num, &powerup_location, &powerup_collected);
        }

        if lives_player == 0 {
                let size = measure_text("Game Over", None, 40, 1.0);
                draw_text("Game Over", screen_width / 2.0 - size.width / 2.0, screen_height / 2.0, 40.0, WHITE);
                let size = measure_text("Press Space to Play Again", None, 25, 1.0);
                draw_text("Press Space to Play Again", screen_width / 2.0 - size.width / 2.0, screen_height / 2.0 + 50.0, 25.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    lives_player = 3;
                    lives_ai = 3;
                    game_started = false;
                }
        } else if lives_ai == 0 {
                let size = measure_text("You Win", None, 40, 1.0);
                draw_text("You Win", screen_width / 2.0 - size.width / 2.0, screen_height / 2.0, 40.0, WHITE);
                let size = measure_text("Press Space to Play Again", None, 25, 1.0);
                draw_text("Press Space to Play Again", screen_width / 2.0 - size.width / 2.0, screen_height / 2.0 + 50.0, 25.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    lives_player = 3;
                    lives_ai = 3;
                    game_started = false;
                }

        } else if !game_started {

            if first_start {
                let mut size = measure_text("Pong with a Twist", None, 40, 1.0);
                draw_text("Pong with a Twist", screen_width / 2.0 - size.width / 2.0, 50.0, 40.0, WHITE);

                draw_text("Press Space to Start", screen_width / 2.0 - 100.0, 350.0, 25.0, WHITE);

                draw_texture(&wasd, 200.0, 100.0, WHITE);
                draw_text("Press W and S to move the paddle", 255.0, 135.0, 20.0, WHITE);

                draw_texture(&space_bar, 137.5, 162.5, WHITE);
                draw_text("Hold Space to move the paddle faster", 255.0, 200.0, 20.0, WHITE);

                draw_texture(&powerup_icon, 207.5, 240.0, WHITE);
                draw_text("PowerUps: Collect PowerUps with your paddle", 255.0, 265.0, 20.0, WHITE);

                /*draw_texture(&powerup_shield, 200.0, 250.0, WHITE);
                draw_text("Shield Powerup: Spawns a shield behind your paddle", 255.0, 275.0, 20.0, WHITE);
                draw_texture(&powerup_speed, 200.0, 300.0, WHITE);
                draw_text("Speed Powerup: Increases the speed of the ball", 255.0, 325.0, 20.0, WHITE);
                draw_text("after hitting your paddle again", 387.0, 350.0, 20.0, WHITE);
                draw_texture(&powerup_size, 200.0, 350.0, WHITE);
                draw_text("Size Powerup: Increases the size of your paddle", 255.0, 375.0, 20.0, WHITE);
                draw_texture(&powerup_multi, 200.0, 400.0, WHITE);
                draw_text("Multi Powerup: Adds a second ball to the game", 255.0, 425.0, 20.0, WHITE);*/
            } else if lives_player >= 0 || lives_ai >= 0 {
                let mut size = measure_text("Press Space to Continue", None, 25, 1.0);
                draw_text("Press Space to Continue", screen_width / 2.0 - size.width / 2.0, 300.0, 25.0, WHITE);
            }
        }
        next_frame().await
    }
}