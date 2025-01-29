use macroquad::prelude::*;

const GRAVITY: f32 = 0.08;
const JUMP_STRENGTH: f32 = -3.0;
const PIPE_WIDTH: f32 = 50.0;
const PIPE_GAP: f32 = 150.0;
const PIPE_SPEED: f32 = 2.0;

struct Bird {
    y: f32,
    velocity: f32,
}

struct Pipe {
    x: f32,
    height: f32,
    passed: bool,
}

impl Bird {
    fn new() -> Self {
        Self { y: screen_height() / 2.0, velocity: 0.0 }
    }
    fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity;
        if is_key_pressed(KeyCode::Space) {
            self.velocity = JUMP_STRENGTH;
        }
    }
    fn draw(&self) {
        draw_circle(screen_width() / 4.0, self.y, 20.0, YELLOW);
    }
}

impl Pipe {
    fn new(x: f32, height: f32) -> Self {
        Self { x, height, passed: false }
    }
    fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }
    fn draw(&self) {
        draw_rectangle(self.x, 0.0, PIPE_WIDTH, self.height, GREEN);
        draw_rectangle(self.x, self.height + PIPE_GAP, PIPE_WIDTH, screen_height(), GREEN);
    }
    fn collides_with(&self, bird: &Bird) -> bool {
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

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut bird = Bird::new();
    let mut pipes = vec![Pipe::new(screen_width(), rand::gen_range(50.0, 300.0))];
    let mut score = 0;

    loop {
        clear_background(SKYBLUE);
        bird.update();
        bird.draw();

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
            score = 0;
        } else {
            pipes = new_pipes;
        }
        
        draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}