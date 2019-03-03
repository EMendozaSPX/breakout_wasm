extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    screen_width: i32,
    screen_height: i32,
    ball: Ball,
    paddle: Paddle,
}

#[wasm_bindgen]
impl Game {
    pub fn new(screen_width: i32, screen_height: i32) -> Game {
        let ball = Ball {
            x: screen_width / 2,
            y: screen_height - 40,
            dx: 2,
            dy: -2,
            radius: 10,
        };

        Game {
            screen_width: screen_width,
            screen_height: screen_height,
            ball: ball,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.ball.x += self.ball.dx * (delta_time / 10.0).floor() as i32;
        self.ball.y += self.ball.dy * (delta_time / 10.0).floor() as i32;

        if self.ball.x + self.ball.dx > self.screen_width - self.ball.radius || self.ball.x + self.ball.dx < self.ball.radius {
            self.ball.dx = -self.ball.dx;
        }

        if self.ball.y + self.ball.dy > self.screen_height - self.ball.radius || self.ball.y + self.ball.dy < self.ball.radius {
            self.ball.dy = -self.ball.dy;
        }
    }

    pub fn ball_x(&self) -> i32 {
        self.ball.x
    }

    pub fn ball_y(&self) -> i32 {
        self.ball.y
    }

    pub fn ball_radius(&self) -> i32 {
        self.ball.radius
    }
}

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    radius: i32,
    color: [i32],
}

struct Paddle {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}
