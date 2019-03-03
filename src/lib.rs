extern crate wasm_bindgen;

extern crate js_sys;

use wasm_bindgen::prelude::*;

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    radius: i32,
    color: (r: i32, g: i32, b: i32),
}

struct Paddle {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (i32, i32, i32) {
    c = v * s;
    k = h / 60;
    x = c * (1 - js_sys::Math::abs(k % 2 - 1));
    let (r1: f64, g1: f64, b1: f64) =
        if 0.0 <= k && k <= 1.0 {
            (c, x, 0.0)
        } else if 1.0 < k && k <= 2.0 {
            (x, c, 0.0)
        } else if 2.0 < k && k <= 3.0 {
            (0.0, c, x)
        } else if 3.0 < k && k <= 4.0 {
            (0.0, x, c)
        } else if 4.0 < k && k <= 5.0 {
            (x, 0.0, c)
        } else if 5.0 < k && k <= 6.0 {
            (c, 0.0, x)
        } else {
            (0.0, 0.0, 0.0)
        }
    m = v - c
    let (r: i32, g: i32, b: i32) = (((r1 + m) * 256.0).floor() as i32, ((g1 + m) * 256.0).floor() as i32, ((b1 + m) * 256.0).floor() as i32)
    (r, g, b)
}

fn random_color() -> (i32, i32, i32) {
    let g_r_c: f64 = 0.618033988749895;
    let mut num = js_sys::Math::random();
    num += g_r_c;
    num %= 1;
    num *= 360.0;
    hsv_to_rgb(num.floor(), 0.5, 0.95);
}

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
            color
        };

        let paddle = Paddle {
            width: 
        }

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
