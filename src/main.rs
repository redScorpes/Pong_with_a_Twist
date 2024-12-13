use std::time::SystemTime;
use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use macroquad::rand::{self, srand};
use macroquad::ui::{hash, root_ui, widgets};

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
        draw_texture(pong_heart, 30.0, 10.0, WHITE);
        draw_texture(pong_heart, 50.0, 10.0, WHITE);
        draw_texture(pong_heart, 70.0, 10.0, WHITE);
    } else if lives_player == 2 {
        draw_texture(pong_heart, 30.0, 10.0, WHITE);
        draw_texture(pong_heart, 50.0, 10.0, WHITE);
    } else if lives_player == 1 {
        draw_texture(pong_heart, 30.0, 10.0, WHITE);
    }
}

fn draw_lives_ai(lives_ai: i32, pong_heart: &Texture2D, screen_width: f32) {
    if lives_ai == 3 {
        draw_texture(pong_heart, screen_width - 85.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 65.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 45.0, 10.0, WHITE);
    } else if lives_ai == 2 {
        draw_texture(pong_heart, screen_width - 85.0, 10.0, WHITE);
        draw_texture(pong_heart, screen_width - 65.0, 10.0, WHITE);
    } else if lives_ai == 1 {
        draw_texture(pong_heart, screen_width - 45.0, 10.0, WHITE);
    }
}

fn draw_powerup(powerup_shield: &Texture2D, powerup_speed: &Texture2D, powerup_size: &Texture2D, powerup_multi: &Texture2D, powerup_num: &i32, powerup_location: &Vec2, powerup_collected: &bool) {
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

struct PauseMenu {
    is_paused: bool,
    volume: f32,
}

impl PauseMenu {
    fn new() -> Self {
        PauseMenu { is_paused: false, volume: 1.0 }
    }

    fn toggle(&mut self) {
        self.is_paused = !self.is_paused;
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn render(
        &mut self,
        lives_player: &mut i32,
        lives_ai: &mut i32,
        game_started: &mut bool,
        ball_pos: &mut Vec2,
        ball_direction: &mut Vec2,
        ball_speed: &mut f32,
        powerup_active: &mut bool,
        powerup_collected: &mut bool,
        shield_time: &mut f32,
        speed_time: &mut f32,
        size_time: &mut f32,
        multi_time: &mut f32,
        second_ball: &mut bool,
        paddle1_hight: &mut f32,
        paddle1_colour: &mut Color,
        screen_width: f32,
        screen_height: f32,
        default_ball_speed: f32,
        two_player: &mut bool,
        paddle1_x: f32,
        paddle1_y: f32,
        paddle1_width: f32,
        paddle2_x: f32,
        paddle2_y: f32,
        paddle2_width: f32,
        paddle2_hight: f32,
        ball_size: Vec2,
        ball_color: Color,
        ball2_pos: Vec2,
        ball2_size: Vec2,
        ball2_color: Color,
        top_barrier: (f32, f32, f32, f32),
        bottom_barrier: (f32, f32, f32, f32),
        pong_heart: &Texture2D,
        powerup_shield: &Texture2D,
        powerup_speed: &Texture2D,
        powerup_size: &Texture2D,
        powerup_multi: &Texture2D,
        powerup_num: &i32,
        powerup_location: &Vec2,
        stamina_bar_time: &mut f32,
        out_of_stamina: &mut bool,
    ) {

        draw_lives_player(*lives_player, pong_heart);
        draw_lives_ai(*lives_ai, pong_heart, screen_width);

        if *powerup_active {
            draw_powerup(&powerup_shield, &powerup_speed, &powerup_size, &powerup_multi, &powerup_num, &powerup_location, &powerup_collected);
        }

        draw_rectangle(top_barrier.0, top_barrier.1, top_barrier.2, top_barrier.3, WHITE);
        draw_rectangle(bottom_barrier.0, bottom_barrier.1, bottom_barrier.2, bottom_barrier.3, WHITE);

        draw_rectangle(paddle1_x, paddle1_y, paddle1_width, *paddle1_hight, *paddle1_colour);
        draw_rectangle(paddle2_x, paddle2_y, paddle2_width, paddle2_hight, WHITE);
        draw_rectangle(ball_pos.x, ball_pos.y, ball_size.x, ball_size.y, ball_color);
        if *second_ball {
            draw_rectangle(ball2_pos.x, ball2_pos.y, ball2_size.x, ball2_size.y, ball2_color);
        }

        draw_stamina_bar(*stamina_bar_time, *out_of_stamina);

        // Render a simple pause menu
        draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::new(0.0, 0.0, 0.0, 0.7));
        let text = "Game Paused\nPress ESC to Resume";
        let text_size = measure_text(text, None, 40, 1.0);
        draw_text(
            text,
            (screen_width - text_size.width) / 2.0,
            50.0,
            40.0,
            WHITE,
        );

        draw_text("Choose Game Mode:", 150.0, 200.0, 30.0, WHITE);
        draw_text("Volume:", 280.0, 275.0, 30.0, WHITE);

        let button_text = "Restart";
        let button_size = measure_text(button_text, None, 30, 1.0);
        let button_x = (screen_width - button_size.width) / 2.0;
        let button_y = 400.0;
        if is_mouse_button_down(MouseButton::Left) && mouse_position().0 >= button_x - 10.0 && mouse_position().0 <= button_x + button_size.width + 10.0 &&
            mouse_position().1 >= button_y - 30.0 && mouse_position().1 <= button_y + 10.0 {
            draw_rectangle(button_x - 10.0, button_y - 30.0, button_size.width + 20.0, 40.0, BLACK);
            draw_text(button_text, button_x, button_y, 30.0, WHITE);
        } else {
            draw_rectangle(button_x - 10.0, button_y - 30.0, button_size.width + 20.0, 40.0, WHITE);
            draw_text(button_text, button_x, button_y, 30.0, BLACK);
        }

        // Check for button click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.0 >= button_x - 10.0 && mouse_pos.0 <= button_x + button_size.width + 10.0 &&
                mouse_pos.1 >= button_y - 30.0 && mouse_pos.1 <= button_y + 10.0 {
                // Restart the game
                restart_game(
                    lives_player,
                    lives_ai,
                    game_started,
                    ball_pos,
                    ball_direction,
                    ball_speed,
                    powerup_active,
                    powerup_collected,
                    shield_time,
                    speed_time,
                    size_time,
                    multi_time,
                    second_ball,
                    paddle1_hight,
                    paddle1_colour,
                    default_ball_speed,
                    paddle1_x,
                    paddle1_y,
                    paddle1_width,
                    stamina_bar_time,
                    out_of_stamina,
                );
            }
        }

        let bot_button_text = "BOT";
        let bot_button_size = measure_text(bot_button_text, None, 30, 1.0);
        let bot_button_x = 400.0;
        let bot_button_y = 200.0;
        if mouse_position().0 >= bot_button_x - 10.0 && mouse_position().0 <= bot_button_x + bot_button_size.width + 10.0 &&
            mouse_position().1 >= bot_button_y - 30.0 && mouse_position().1 <= bot_button_y + 10.0 {
            draw_rectangle(bot_button_x - 10.0, bot_button_y - 30.0, bot_button_size.width + 20.0, 40.0, BLACK);
            draw_text(bot_button_text, bot_button_x, bot_button_y, 30.0, WHITE);
        } else {
            draw_rectangle(bot_button_x - 10.0, bot_button_y - 30.0, bot_button_size.width + 20.0, 40.0, WHITE);
            draw_text(bot_button_text, bot_button_x, bot_button_y, 30.0, BLACK);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.0 >= bot_button_x - 10.0 && mouse_pos.0 <= bot_button_x + bot_button_size.width + 10.0 &&
                mouse_pos.1 >= bot_button_y - 30.0 && mouse_pos.1 <= bot_button_y + 10.0 {
                *two_player = false;
            }
        }

        if !*two_player {
            let border_thickness = 2.0;
            draw_line(bot_button_x - 20.0, bot_button_y - 40.0, bot_button_x + bot_button_size.width + 20.0, bot_button_y - 40.0, border_thickness, WHITE);
            draw_line(bot_button_x - 20.0, bot_button_y + 20.0, bot_button_x + bot_button_size.width + 20.0, bot_button_y + 20.0, border_thickness, WHITE);
            draw_line(bot_button_x - 20.0, bot_button_y - 40.0, bot_button_x - 20.0, bot_button_y + 20.0, border_thickness, WHITE);
            draw_line(bot_button_x + bot_button_size.width + 20.0, bot_button_y - 40.0, bot_button_x + bot_button_size.width + 20.0, bot_button_y + 20.0, border_thickness, WHITE);
        }

        let two_player_button_text = "2 Player";
        let two_player_button_size = measure_text(two_player_button_text, None, 30, 1.0);
        let two_player_button_x = 500.0;
        let two_player_button_y = 200.0;
        if mouse_position().0 >= two_player_button_x - 10.0 && mouse_position().0 <= two_player_button_x + two_player_button_size.width + 10.0 &&
            mouse_position().1 >= two_player_button_y - 30.0 && mouse_position().1 <= two_player_button_y + 10.0 {
            draw_rectangle(two_player_button_x - 10.0, two_player_button_y - 30.0, two_player_button_size.width + 20.0, 40.0, BLACK);
            draw_text(two_player_button_text, two_player_button_x, two_player_button_y, 30.0, WHITE);
        } else {
            draw_rectangle(two_player_button_x - 10.0, two_player_button_y - 30.0, two_player_button_size.width + 20.0, 40.0, WHITE);
            draw_text(two_player_button_text, two_player_button_x, two_player_button_y, 30.0, BLACK);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.0 >= two_player_button_x - 10.0 && mouse_pos.0 <= two_player_button_x + two_player_button_size.width + 10.0 &&
                mouse_pos.1 >= two_player_button_y - 30.0 && mouse_pos.1 <= two_player_button_y + 10.0 {
                *two_player = true;
            }
        }

        if *two_player {
            let border_thickness = 2.0;
            draw_line(two_player_button_x - 20.0, two_player_button_y - 40.0, two_player_button_x + two_player_button_size.width + 20.0, two_player_button_y - 40.0, border_thickness, WHITE);
            draw_line(two_player_button_x - 20.0, two_player_button_y + 20.0, two_player_button_x + two_player_button_size.width + 20.0, two_player_button_y + 20.0, border_thickness, WHITE);
            draw_line(two_player_button_x - 20.0, two_player_button_y - 40.0, two_player_button_x - 20.0, two_player_button_y + 20.0, border_thickness, WHITE);
            draw_line(two_player_button_x + two_player_button_size.width + 20.0, two_player_button_y - 40.0, two_player_button_x + two_player_button_size.width + 20.0, two_player_button_y + 20.0, border_thickness, WHITE);
        }

        widgets::Group::new(hash!(), vec2(200.0, 50.0))
            .position(vec2(400.0, 235.0))
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "");
                ui.slider(hash!(), "", 0.0..1.0, &mut self.volume);
            });

        self.get_volume();

    }
}

fn restart_game(
    lives_player: &mut i32,
    lives_ai: &mut i32,
    game_started: &mut bool,
    ball_pos: &mut Vec2,
    ball_direction: &mut Vec2,
    ball_speed: &mut f32,
    powerup_active: &mut bool,
    powerup_collected: &mut bool,
    shield_time: &mut f32,
    speed_time: &mut f32,
    size_time: &mut f32,
    multi_time: &mut f32,
    second_ball: &mut bool,
    paddle1_hight: &mut f32,
    paddle1_colour: &mut Color,
    default_ball_speed: f32,
    paddle1_x: f32,
    paddle1_y: f32,
    paddle1_width: f32,
    stamina_bar_time: &mut f32,
    out_of_stamina: &mut bool,
) {
    *lives_player = 3;
    *lives_ai = 3;
    *game_started = false;
    *ball_pos = vec2(paddle1_x + paddle1_width + 5.0, paddle1_y + *paddle1_hight / 2.0 - 5.0);
    *ball_direction = vec2(0.5, 0.5);
    *ball_speed = default_ball_speed;
    *powerup_active = false;
    *powerup_collected = false;
    *shield_time = 0.0;
    *speed_time = 0.0;
    *size_time = 0.0;
    *multi_time = 0.0;
    *second_ball = false;
    *paddle1_hight = 70.0;
    *paddle1_colour = WHITE;
    *stamina_bar_time = 5.0;
    *out_of_stamina = false;
}

fn draw_stamina_bar (stamina_bar_time: f32, out_of_stamina: bool) {
    let screen_width = window_conf().window_width as f32;

    let stamina_bar_width = 1200.0;
    let stamina_bar_height = 10.0;

    let stamina_bar_x = (screen_width - stamina_bar_width) / 2.0;
    let stamina_bar_y = 10.0;

    let stamina_bar_fill = (stamina_bar_time * (stamina_bar_width / 5.0)) / 2.0;

    draw_rectangle(stamina_bar_x + (stamina_bar_width - stamina_bar_fill) / 2.0, stamina_bar_y, stamina_bar_fill, stamina_bar_height, WHITE);

    if out_of_stamina {
        let blink = (get_time() * 10.0).sin() > 0.0;
        draw_rectangle(stamina_bar_x + (stamina_bar_width - stamina_bar_fill) / 2.0, stamina_bar_y, stamina_bar_fill, stamina_bar_height, if blink { WHITE } else { BLACK });
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut pause_menu = PauseMenu::new();

    let pong_heart = load_texture("assets/Pong_Heart.png").await.unwrap();

    let screen_width = window_conf().window_width as f32;
    let screen_height = window_conf().window_height as f32;


    let mut paddle1_hight: f32 = 70.0;
    let paddle1_width: f32 = 15.0;
    let mut paddle1_x: f32 = 10.0;
    let mut paddle1_y: f32 = 100.0;
    let mut paddle1_speed: f32 = 200.0;
    let mut paddle1_colour: Color = WHITE;

    let mut paddle2_hight: f32 = 70.0;
    let mut paddle2_width: f32 = 15.0;
    let mut paddle2_x: f32 = screen_width - 25.0;
    let mut paddle2_y: f32 = 100.0;
    let mut paddle2_speed: f32 = 300.0;
    let mut paddle2_player_speed: f32 = paddle1_speed.clone();

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

    let powerup_shield = load_texture("assets/Pong_PowerUp_Shield.png").await.unwrap();
    let powerup_speed = load_texture("assets/Pong_PowerUp_Speed.png").await.unwrap();
    let powerup_size = load_texture("assets/Pong_PowerUp_Size.png").await.unwrap();
    let powerup_multi = load_texture("assets/Pong_PowerUp_Mult.png").await.unwrap();
    let powerup_icon = load_texture("assets/Pong_PowerUp.png").await.unwrap();

    let wasd = load_texture("assets/Pong_wasd.png").await.unwrap();
    let space_bar = load_texture("assets/Pong_SpaceBar.png").await.unwrap();
    let esc = load_texture("assets/Pong_esc.png").await.unwrap();

    let mut powerup_collected = false;
    let mut powerup_active = false;

    srand(miniquad::date::now() as u64);
    let mut powerup_num = rand::gen_range(0, 4);
    let mut powerup_location = vec2(800.0, 600.0);

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

    let mut two_player = false;

    let mut stamina_bar_time = 3.0;
    let mut out_of_stamina = false;


    loop {

        if is_key_pressed(KeyCode::Escape) {
            pause_menu.toggle();
        }

        clear_background(BLACK);

        if pause_menu.is_paused {
            pause_menu.render(
                &mut lives_player,
                &mut lives_ai,
                &mut game_started,
                &mut ball_pos,
                &mut ball_direction,
                &mut ball_speed,
                &mut powerup_active,
                &mut powerup_collected,
                &mut shield_time,
                &mut speed_time,
                &mut size_time,
                &mut multi_time,
                &mut second_ball,
                &mut paddle1_hight,
                &mut paddle1_colour,
                screen_width,
                screen_height,
                default_ball_speed,
                &mut two_player,
                paddle1_x,
                paddle1_y,
                paddle1_width,
                paddle2_x,
                paddle2_y,
                paddle2_width,
                paddle2_hight,
                ball_size,
                ball_color,
                ball2_pos,
                ball2_size,
                ball2_color,
                top_barrier,
                bottom_barrier,
                &pong_heart,
                &powerup_shield,
                &powerup_speed,
                &powerup_size,
                &powerup_multi,
                &powerup_num,
                &powerup_location,
                &mut stamina_bar_time,
                &mut out_of_stamina,
            );
        } else {
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
                    powerup_location.x -= 150.0 * delta_time;
                }

                if shield_time > 0.0 {
                    shield_time -= delta_time;
                    if shield_time <= 2.0 {
                        countdown_time -= delta_time;
                        if countdown_time <= 0.0 {
                            play_sound(&countdown, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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
                            play_sound(&countdown, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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

                if powerup_collected && powerup_num == 2 {
                    if size_time <= 0.0 {
                        if paddle1_hight == 140.0 {
                            paddle1_y += 25.0;
                        } else if paddle1_hight == 190.0 {
                            paddle1_hight += 50.0;
                        } else if paddle1_hight == 240.0{
                            paddle1_hight += 75.0;
                        }
                    }
                }

                if multi_time > 0.0 {
                    multi_time -= delta_time;
                    if multi_time <= 2.0 {
                        countdown_time -= delta_time;
                        if countdown_time <= 0.0 {
                            play_sound(&countdown, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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
                    play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                }

                if rects_collide(vec2(bottom_barrier.0, bottom_barrier.1), vec2(bottom_barrier.2, bottom_barrier.3), ball_pos, ball_size) {
                    ball_direction.y = -ball_direction.y;
                    ball_pos.y = bottom_barrier.1 - ball_size.y;
                    play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                }
                if second_ball {
                    if rects_collide(vec2(top_barrier.0, top_barrier.1), vec2(top_barrier.2, top_barrier.3), ball2_pos, ball2_size) {
                        ball2_direction.y = -ball2_direction.y;
                        ball2_pos.y = top_barrier.1 + top_barrier.3;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    } else if rects_collide(vec2(bottom_barrier.0, bottom_barrier.1), vec2(bottom_barrier.2, bottom_barrier.3), ball2_pos, ball2_size) {
                        ball2_direction.y = -ball2_direction.y;
                        ball2_pos.y = bottom_barrier.1 - ball2_size.y;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    }
                }

                if shield_time > 0.0 {
                    if rects_collide(vec2(shield.0, shield.1), vec2(shield.2, shield.3), ball_pos, ball_size) {
                        ball_direction.x = -ball_direction.x;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    } else if second_ball && rects_collide(vec2(shield.0, shield.1), vec2(shield.2, shield.3), ball2_pos, ball2_size) {
                        ball2_direction.x = -ball2_direction.x;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    }
                }

                if ball_pos.x <= 0.0 || ball_pos.x + ball_size.x >= screen_width || second_ball && ball2_pos.x <= 0.0 || second_ball && ball2_pos.x + ball2_size.x >= screen_width {
                    play_sound(&goal_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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
                    stamina_bar_time = 5.0;
                    out_of_stamina = false;
                }

                if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), ball_pos, ball_size) {
                    if ball_pos.y < paddle1_y || ball_pos.y > paddle1_y + paddle1_hight {
                        ball_direction.y = -ball_direction.y;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    } else {
                        ball_direction.x = -ball_direction.x;
                        ball_pos.x = paddle1_x + paddle1_width;
                        paddle1_hit = true;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    }
                }
                if second_ball {
                    if rects_collide(vec2(paddle1_x, paddle1_y), vec2(paddle1_width, paddle1_hight), ball2_pos, ball2_size) {
                        if ball2_pos.y < paddle1_y || ball2_pos.y > paddle1_y + paddle1_hight {
                            ball2_direction.y = -ball2_direction.y;
                            play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                        } else {
                            ball2_direction.x = -ball2_direction.x;
                            ball2_pos.x = paddle1_x + paddle1_width;
                            paddle1_hit = true;
                            play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                        }
                    }
                }
                if rects_collide(vec2(paddle2_x, paddle2_y), vec2(paddle2_width / 4.0, paddle2_hight), ball_pos, ball_size) {
                    if ball_pos.y < paddle2_y || ball_pos.y > paddle2_y + paddle2_hight {
                        ball_direction.y = -ball_direction.y;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                    } else {
                        ball_direction.x = -ball_direction.x;
                        ball_pos.x = paddle2_x - ball_size.x;
                        play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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
                            play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                        } else {
                            ball2_direction.x = -ball2_direction.x;
                            ball2_pos.x = paddle2_x - ball2_size.x;
                            play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
                        }
                        if !powerup_collected && !powerup_active {
                            powerup_location = vec2(paddle2_x - 20.0, paddle2_y + paddle2_hight / 2.0 - 25.0);
                            play_sound(&pong_hit, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });
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
                    play_sound(&powerup_collect, PlaySoundParams { volume: pause_menu.get_volume(), ..Default::default() });


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
                if !out_of_stamina {
                    stamina_bar_time -= delta_time * 2.0;
                    paddle1_speed = 400.0;
                } else {
                    if stamina_bar_time < 5.0 {
                        stamina_bar_time += delta_time;
                    }
                    paddle1_speed = 200.0;
                }
            } else {
                if stamina_bar_time < 5.0 {
                    stamina_bar_time += delta_time;
                }
                paddle1_speed = 200.0;
            }

            if stamina_bar_time <= 0.0 {
                stamina_bar_time = 0.0;
                out_of_stamina = true;
            } else if stamina_bar_time >= 5.0 {
                stamina_bar_time = 5.0;
                out_of_stamina = false;
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


            if !two_player {
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
            } else {
                if is_key_down(KeyCode::RightShift) {
                    paddle2_player_speed = 400.0;
                } else {
                    paddle2_player_speed = 200.0;
                }

                if paddle2_y > 0.0 + 10.0 {
                    if is_key_down(KeyCode::Up) {
                        paddle2_y -= paddle2_player_speed * delta_time;
                    }
                };

                if paddle2_y < screen_height - paddle2_hight - 10.0 {
                    if is_key_down(KeyCode::Down) {
                        paddle2_y += paddle2_player_speed * delta_time;
                    }
                }
            }



            draw_rectangle(paddle1_x, paddle1_y, paddle1_width, paddle1_hight, paddle1_colour);
            draw_rectangle(paddle2_x, paddle2_y, paddle2_width, paddle2_hight, WHITE);

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

            draw_stamina_bar(stamina_bar_time, out_of_stamina);

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

                    draw_text("Press Space to Start", screen_width / 2.0 - 100.0, 395.0, 25.0, WHITE);

                    draw_texture(&wasd, 200.0, 100.0, WHITE);
                    draw_text("Press W and S to move the paddle", 255.0, 135.0, 20.0, WHITE);

                    draw_texture(&space_bar, 137.5, 162.5, WHITE);
                    draw_text("Hold Space to move the paddle faster", 255.0, 200.0, 20.0, WHITE);

                    draw_texture(&esc, 207.5, 240.0, WHITE);
                    draw_text("Press ESC to open the Pause-Menu", 255.0, 265.0, 20.0, WHITE);

                    draw_texture(&powerup_icon, 207.5, 305.0, WHITE);
                    draw_text("PowerUps: Collect PowerUps with your paddle", 255.0, 330.0, 20.0, WHITE);

                } else if lives_player >= 0 || lives_ai >= 0 {
                    let mut size = measure_text("Press Space to Continue", None, 25, 1.0);
                    draw_text("Press Space to Continue", screen_width / 2.0 - size.width / 2.0, 300.0, 25.0, WHITE);
                }
            }
        }
        next_frame().await
    }
}