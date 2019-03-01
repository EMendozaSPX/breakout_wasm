extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub struct Game {
    ScreenWidth: i32,
    ScreenHeight: i32,
    ball: Ball,
}

impl Game {
    pub fn new(screen_width: i32, screen_height: i32) -> Game {
        let ball = Ball {
            screen_width / 2,
            screen_height - 40,
            2,
            -2,
            10,
        };

        Game {
            screen_width,
            screen_heigh,
            ball,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.ball.x += self.ball.dx * (delta_time / 10).floor() as i32;
        self.ball.y += self.ball.dy * (delta_time / 10).floor() as i32;

        if self.ball.x + self.ball.dx > self.screenWidth - self.ball.radius || self.ball.x + self.ball.dx < self.ball.raidius {
            self.ball.dx = -self.ball.dx;
        }

        if self.ball.y + self.ball.dy > self.screenHeight - self.ball.radius || self.ball.y + self.ball.dy < self.ball.raidius {
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

#[wasm_bindgen]
pub struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    radius: i32,
}

#[wasm_bindgen]
impl Ball {
    pub fn new(screen_width: i32, screen_height: i32, dx: i32, dy: i32) -> Ball {
        let x = screen_width / 2;
        let y = screen_height - 40;
        let ballRadius = 10;
        Ball {
            x,
            y,
            dx,
            dy,
            ballRadius,
        }
    }

    pub fn update(&mut self, deltaTime: f32) {
        self.x += self.dx * (deltaTime / 10.0).floor() as i32;
        self.y += self.dy * (deltaTime / 10.0).floor() as i32;

        if self.x + self.dx > self.screenWidth - self.ballRadius || self.x + self.dx < self.ballRadius {
            self.dx = -self.dx;
        }

        if self.y + self.dy > self.screenHeight - self.ballRadius || self.y + self.dy < self.ballRadius {
            self.dy = -self.dy;
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_radius(&self) -> i32 {
        self.ballRadius
    }
}