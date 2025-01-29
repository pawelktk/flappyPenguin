use config::PIPE_WIDTH;
use entities::{Bird, Pipe};
use macroquad::prelude::*;

mod config;
mod entities;

enum GameState {
    MainMenu,
    Playing,
}

#[macroquad::main("Flappy Bird")]
async fn main() {
    let penguin_texture = load_texture("assets/penguin.png").await.unwrap();
    penguin_texture.set_filter(FilterMode::Nearest);
    let duck_texture = load_texture("assets/duck.png").await.unwrap();
    duck_texture.set_filter(FilterMode::Nearest);
    let mut duck = false;
    let mut state = GameState::MainMenu;
    let mut bird = Bird::new();
    let mut pipes = vec![Pipe::new(screen_width(), rand::gen_range(50.0, 300.0))];
    let mut score = 0;
    let mut high_score = 0;
    let mut beaten = false;

    loop {
        match state {
            GameState::MainMenu => {
                clear_background(SKYBLUE);
                draw_text(
                    "Flappy Penguin",
                    screen_width() / 2.0 - 120.0,
                    screen_height() / 2.0 - 40.0,
                    40.0,
                    WHITE,
                );
                draw_text(
                    "Press SPACE to Start",
                    screen_width() / 2.0 - 130.0,
                    screen_height() / 2.0,
                    30.0,
                    WHITE,
                );
                draw_text(
                    &format!("High score: {}", high_score),
                    20.0,
                    40.0,
                    30.0,
                    WHITE,
                );
                if beaten {
                    draw_text(
                        "You've beaten your high score!",
                        screen_width() / 2.0 - 180.0,
                        screen_height() / 2.0 + 40.0,
                        30.0,
                        RED,
                    );
                }
                if duck {
                    draw_texture(
                        &duck_texture,
                        screen_width() / 2.0 - 20.0,
                        screen_height() / 2.0 + 80.0,
                        WHITE,
                    );
                } else {
                    draw_texture(
                        &penguin_texture,
                        screen_width() / 2.0 - 20.0,
                        screen_height() / 2.0 + 80.0,
                        WHITE,
                    );
                }
                draw_text(
                    "Choose your hero with arrow keys",
                    screen_width() / 2.0 - 140.0,
                    screen_height() / 2.0 + 140.0,
                    20.0,
                    YELLOW,
                );
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Left) {
                    duck = !duck;
                }
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                    beaten = false;
                }
                next_frame().await;
            }
            GameState::Playing => {
                clear_background(SKYBLUE);
                bird.update();
                //bird.draw();
                if duck {
                    bird.draw(&duck_texture);
                } else {
                    bird.draw(&penguin_texture);
                }
                if let Some(last_pipe) = pipes.last() {
                    if last_pipe.x < screen_width() - 200.0 {
                        pipes.push(Pipe::new(screen_width(), rand::gen_range(50.0, 300.0)));
                    }
                }

                // Collect indices of pipes that should be removed
                let mut reset_game = false;
                let mut new_pipes = Vec::new();

                for pipe in &mut pipes {
                    pipe.update();
                    pipe.draw();

                    if pipe.collides_with(&bird) || bird.y > screen_height() {
                        reset_game = true;
                    } else if pipe.x + PIPE_WIDTH > 0.0 {
                        new_pipes.push(Pipe::new(pipe.x, pipe.height));
                    }
                }

                score += 1;

                if reset_game {
                    bird = Bird::new();
                    pipes = vec![Pipe::new(screen_width(), rand::gen_range(50.0, 300.0))];
                    if score > high_score {
                        high_score = score;
                        beaten = true;
                    }
                    score = 0;
                    state = GameState::MainMenu;
                } else {
                    pipes = new_pipes;
                }

                draw_text(&format!("Score: {}", score), 20.0, 80.0, 30.0, WHITE);
                draw_text(
                    &format!("High score: {}", high_score),
                    20.0,
                    40.0,
                    30.0,
                    WHITE,
                );

                next_frame().await;
            }
        }
    }
}
