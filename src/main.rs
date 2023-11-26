use std::process::exit;

use macroquad::{color::Color, ui::root_ui, math::vec2, text::{TextParams, Font}};

use device_query::{DeviceState, Keycode};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Input {
    pub device_input: DeviceState,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            device_input: DeviceState::new(),
        }
    }
}

pub fn keymap(input: Input) -> Vec<Keycode> {
    let keys: Vec<Keycode> = input.device_input.query_keymap();

    keys
}
#[allow(dead_code)]
#[derive(Clone)]
struct Food {
    x: f32,
    y: f32,
    random_thread: ThreadRng,
    snake: Snake,
    size: [f32; 2],
    is_alive: bool,
    color: Color,
    score: u8
}

impl Default for Food {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            random_thread: rand::thread_rng(),
            snake: Snake::default(),
            size: Snake::default().size,
            is_alive: false,
            color: Color {
                r: 255.,
                g: 255.,
                b: 0.,
                a: 255.,
            },
            score: 1,
        }
    }
}

impl Food {
    pub fn food(&mut self) {
        if !self.is_alive {
            self.x = self.random_thread.gen_range(0.0..self.snake.WINDOW_SIZE[0]);
            self.y = self.random_thread.gen_range(0.0..self.snake.WINDOW_SIZE[1]);

            self.is_alive = true;
        }

        macroquad::shapes::draw_rectangle(self.x, self.y, self.size[0], self.size[1], self.color);
        
        //dont kill me
        let tolerance = -10..=10;
        if tolerance.contains(&(self.y.round() as i32 - self.snake.y.round() as i32)) && tolerance.contains(&(self.x.round() as i32 - self.snake.x.round() as i32)) {
            self.score += 1;
            self.is_alive = false;
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Clone)]
struct Snake {
    x: f32,
    y: f32,
    xy_diff: i8,
    positions: Vec<[f32; 2]>,
    direction: char,
    color: Color,
    size: [f32; 2],
    input: Input,
    WINDOW_SIZE: Vec<f32>,
    is_alive: bool,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            xy_diff: 1,
            positions: Vec::new(),
            direction: 'r',
            color: Color {
                r: 255.,
                g: 0.,
                b: 0.,
                a: 255.,
            },
            size: [10., 10.],
            input: Input::default(),
            WINDOW_SIZE: vec![
                macroquad::prelude::screen_width(),
                macroquad::prelude::screen_height(),
            ],
            is_alive: true,
        }
    }
}

impl Snake {
    pub fn snake(&mut self, score : usize) {
        //increment values, dep. on direction
        if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Left) && self.direction != 'r'
        {
            self.direction = 'l'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Right) && self.direction != 'l'
        {
            self.direction = 'r'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Up) && self.direction != 'u'
        {
            self.direction = 'u'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Down) && self.direction != 'u'
        {
            self.direction = 'd'
        }

        if self.direction == 'l'  {
            self.x -= self.xy_diff as f32;
        } else if self.direction == 'r'  {
            self.x += self.xy_diff as f32;
        } else if self.direction == 'u'  {
            self.y -= self.xy_diff as f32;
        } else if self.direction == 'd'  {
            self.y += self.xy_diff as f32;
        }

        if self.x > self.WINDOW_SIZE[0] {
            self.x = 0.
        }

        if self.y > self.WINDOW_SIZE[1] {
            self.y = 0.
        }
        if self.y < 0. - self.size[1] {
            self.y = self.WINDOW_SIZE[1]
        }
        if self.x < 0. - self.size[0] {
            self.x = self.WINDOW_SIZE[0]
        }

        if self.positions.contains(&[self.x, self.y]) {
            self.is_alive = false;
        }

        self.positions.push([self.x, self.y]);

        if self.positions.len() >= score * 100 {
            self.positions.remove(0);
        }

        //Draw
        for pos in &self.positions {
            macroquad::shapes::draw_rectangle(
                pos[0],
                pos[1],
                self.size[0],
                self.size[1],
                self.color,
            );
        }

    }
}

async fn redraw() {
    macroquad::prelude::next_frame().await;
}

async fn draw_ui(food_struct: &Food) {
    macroquad::text::draw_text(&(food_struct.score - 1).to_string(), 100., 100., 50., macroquad::color::WHITE);
}

#[macroquad::main("main_menu")]
async fn main() {
    
    loop {
        
        /*redraw().await;

        let mut ui = root_ui();
        
        let start = ui.button(vec2(macroquad::prelude::screen_width() / 2., macroquad::prelude::screen_height() / 2.,), "Start");

        if start {
            
        } */

        game_main().await;

    }

}

async fn game_main() {
    let mut snake_struct = Snake::default();

    let mut food_struct = Food::default();

    loop {

        draw_ui(&food_struct).await;

        food_struct.snake = snake_struct.clone();

        //redraw screen
        redraw().await;
        
        Food::food(&mut food_struct);
        Snake::snake(&mut snake_struct, food_struct.score.into());

        if !snake_struct.is_alive {
            exit(0);
        }

    }
}