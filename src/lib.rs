extern crate wasm_bindgen;

extern crate js_sys;

use wasm_bindgen::prelude::*;

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    inc: f32,
    radius: i32,
    color: (i32, i32, i32),
}

struct Paddle {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (i32, i32, i32) {
    let c = v * s;
    let k = h / 60.0;
    let x = c * (1.0 - js_sys::Math::abs(k % 2.0 - 1.0));
    let (r1, g1, b1) =
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
        };
    let m = v - c;
    (((r1 + m) * 256.0).floor() as i32, ((g1 + m) * 256.0).floor() as i32, ((b1 + m) * 256.0).floor() as i32)
}

fn random_color() -> (i32, i32, i32) {
    let g_r_c: f64 = 0.618033988749895;
    let mut num = js_sys::Math::random();
    num += g_r_c;
    num %= 1.0;
    num *= 360.0;
    hsv_to_rgb(num.floor(), 0.5, 0.95)
}

#[wasm_bindgen]
pub struct Game {
    screen_width: i32,
    screen_height: i32,
    lives: i32,
    play: bool,
    right_pressed: bool,
    left_pressed: bool,
    ball: Ball,
    paddle: Paddle,
}

#[wasm_bindgen]
impl Game {
    pub fn new(screen_width: i32, screen_height: i32) -> Game {
        let ball = Ball {
            x: screen_width / 2,
            y: screen_height - 50,
            dx: 2,
            dy: -2,
            inc: 0.0,
            radius: 10,
            color: random_color(),
        };

        let paddle = Paddle {
            width: 150,
            height: 20,
            x: (screen_width - 150) / 2,
            y: screen_height - 40,
        };

        Game {
            screen_width: screen_width,
            screen_height: screen_height,
            play: false,
            right_pressed: false,
            left_pressed: false,
            lives: 3,
            ball: ball,
            paddle: paddle,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.ball.x += self.ball.dx * (delta_time / 10.0).floor() as i32;
        self.ball.y += self.ball.dy * (delta_time / 10.0).floor() as i32;
        
        if self.right_pressed {
            self.paddle.x += 10;
        } else if self.left_pressed {
            self.paddle.x -= 10;
        }

        if self.paddle.x <= 0 {
            self.paddle.x = 0;
        } else if self.paddle.x >= self.screen_width - self.paddle.width {
            self.paddle.x = self.screen_width - self.paddle.width;
        }
        

        if self.ball.x + self.ball.dx > self.screen_width - self.ball.radius || self.ball.x + self.ball.dx < self.ball.radius {
            self.ball.color = random_color();
            self.ball.dx = -self.ball.dx;
        }

        if self.ball.y + self.ball.dy < self.ball.radius {
            self.ball.color = random_color();
            self.ball.dy = -self.ball.dy;
        } else if self.ball.x > self.paddle.x && self.ball.x < self.paddle.x + self.paddle.width {
            if self.ball.y + self.ball.dy > self.paddle.y - self.ball.radius {
                self.ball.dy = -self.ball.dy;
                if self.ball.inc < 5.0 {
                    self.ball.inc += 0.2;
                    if self.ball.inc % 1.0 == 0.0 {
                        if self.ball.dx > 0 {
                            self.ball.dx += 1;
                        } else if self.ball.dx < 0 {
                            self.ball.dx -= 1;
                        }
                        if self.ball.dy > 0 {
                            self.ball.dy += 1;
                        } else if self.ball.dy < 0 {
                            self.ball.dy -= 1;
                        }
                    }
                }
            }
        } else if self.ball.y + self.ball.dy > self.screen_height - self.ball.radius {
            self.lives -= 1;
            self.reset();
        }
    }

    pub fn set_rpressed(&mut self, pressed: bool) {
        self.right_pressed = pressed;
    }

    pub fn set_lpressed(&mut self, pressed: bool) {
        self.left_pressed = pressed;
    }

    pub fn get_lives(&self) -> i32 {
        self.lives
    }

    pub fn set_play(&mut self, play: bool) {
        self.play = play;
    }

    pub fn get_play(&self) -> bool {
        self.play
    }

    pub fn ball_x(&self) -> i32 {
        self.ball.x
    }

    pub fn ball_y(&self) -> i32 {
        self.ball.y
    }

    pub fn ball_dx(&self) -> i32 {
        self.ball.dx
    }

    pub fn ball_radius(&self) -> i32 {
        self.ball.radius
    }

    pub fn ball_color(&self) -> String {
        let (r, g, b) = self.ball.color;
        format!("rgb({}, {}, {})", r, g, b)
    }

    pub fn paddle_width(&self) -> i32 {
        self.paddle.width
    }

    pub fn paddle_height(&self) -> i32 {
        self.paddle.height
    }

    pub fn paddle_x(&self) -> i32 {
        self.paddle.x
    }

    pub fn paddle_y(&self) -> i32 {
        self.paddle.y
    }
}

impl Game {
    fn reset(&mut self) {
        self.play = false;
        self.ball = Ball {
            x: self.screen_width / 2,
            y: self.screen_height - 50,
            dx: 2,
            dy: -2,
            inc: 0.0,
            radius: 10,
            color: random_color(),
        };

        self.paddle = Paddle {
            width: 150,
            height: 20,
            x: (self.screen_width - 150) / 2,
            y: self.screen_height - 40,
        };
    }
}
