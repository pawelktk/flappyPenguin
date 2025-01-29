use macroquad::prelude::*;

use crate::config::{GRAVITY, JUMP_STRENGTH, PIPE_GAP, PIPE_SPEED, PIPE_WIDTH};

pub struct Bird {
    pub y: f32,
    pub velocity: f32,
}

pub struct Pipe {
    pub x: f32,
    pub height: f32,
    pub passed: bool,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            y: screen_height() / 2.0,
            velocity: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity;
        if is_key_pressed(KeyCode::Space) {
            self.velocity = JUMP_STRENGTH;
        }
    }
    /*pub fn draw(&self) {
        draw_circle(screen_width() / 4.0, self.y, 20.0, YELLOW);
    }*/
    pub fn draw(&self, texture: &Texture2D) {
        draw_texture(texture, screen_width() / 4.0 - 20.0, self.y - 20.0, WHITE);
    }
}

impl Pipe {
    pub fn new(x: f32, height: f32) -> Self {
        Self {
            x,
            height,
            passed: false,
        }
    }
    pub fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }
    pub fn draw(&self) {
        draw_rectangle(self.x, 0.0, PIPE_WIDTH, self.height, GREEN);
        draw_rectangle(
            self.x,
            self.height + PIPE_GAP,
            PIPE_WIDTH,
            screen_height(),
            GREEN,
        );
    }
    pub fn collides_with(&self, bird: &Bird) -> bool {
        let bird_x = screen_width() / 4.0;
        let bird_radius = 20.0;
        if bird_x + bird_radius > self.x && bird_x - bird_radius < self.x + PIPE_WIDTH {
            if bird.y - bird_radius < self.height || bird.y + bird_radius > self.height + PIPE_GAP {
                return true;
            }
        }
        false
    }
}
