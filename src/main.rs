use std::process::exit;

use bsod::bsod;
use macroquad::{
    color::Color,
    math::vec2,
    rand::RandomRange,
    text::{Font, TextParams},
    ui::root_ui,
};

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
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct Food {
    x: f32,
    y: f32,
    #[serde(skip)]
    random_thread: ThreadRng,
    snake: Snake,
    size: [f32; 2],
    is_alive: bool,
    #[serde(skip)]
    color: Color,
    score: u8,
    is_special: bool,
    speed_boost: f32,
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
            is_special: false,
            speed_boost: 1.,
        }
    }
}

impl Food {
    pub fn food(&mut self) {
        if !self.is_alive {
            loop {
                self.x = self
                    .random_thread
                    .gen_range(0.0..macroquad::prelude::screen_width());
                self.y = self
                    .random_thread
                    .gen_range(0.0..macroquad::prelude::screen_height());

                if self.snake.positions.iter().any(|f| *f == [self.x, self.y]) {
                    continue;
                }

                break;
            }

            self.is_alive = true;
        }

        if self.score % 10 == 0 {
            self.is_special = true;
        } else {
            self.is_special = false;
        }

        match self.is_special {
            true => {
                macroquad::shapes::draw_rectangle(
                    self.x,
                    self.y,
                    self.size[0],
                    self.size[1],
                    Color {
                        r: 0.,
                        g: 255.,
                        b: 0.,
                        a: 255.,
                    },
                );
            }
            false => {
                macroquad::shapes::draw_rectangle(
                    self.x,
                    self.y,
                    self.size[0],
                    self.size[1],
                    self.color,
                );
            }
        }
        //dont kill me
        let tolerance = -self.size[0] * self.speed_boost..=self.size[1] * self.speed_boost;
        if tolerance.contains(&(self.y.round() - self.snake.y.round()))
            && tolerance.contains(&(self.x.round() - self.snake.x.round()))
        {
            if self.is_special {
                self.score += 5;
                self.speed_boost += 0.3;
            } else {
                self.score += 1;
            }
            self.is_alive = false;
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct Snake {
    x: f32,
    y: f32,
    xy_diff: i8,
    positions: Vec<[f32; 2]>,
    direction: char,
    #[serde(skip)]
    color: Color,
    size: [f32; 2],
    #[serde(skip)]
    input: Input,
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
            is_alive: true,
        }
    }
}

impl Snake {
    pub fn snake(&mut self, score: usize, speed_boost: f32) {
        //increment values, dep. on direction
        if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Left)
            && self.direction != 'r'
        {
            self.direction = 'l'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Right)
            && self.direction != 'l'
        {
            self.direction = 'r'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Up)
            && self.direction != 'd'
        {
            self.direction = 'u'
        } else if self
            .input
            .device_input
            .query_keymap()
            .contains(&device_query::Keycode::Down)
            && self.direction != 'u'
        {
            self.direction = 'd'
        }

        match self.direction {
            'l' => self.x -= self.xy_diff as f32 * speed_boost,
            'r' => self.x += self.xy_diff as f32 * speed_boost,
            'u' => self.y -= self.xy_diff as f32 * speed_boost,
            'd' => self.y += self.xy_diff as f32 * speed_boost,
            _ => {
                panic!("What the fuck")
            }
        }

        if self.x > macroquad::prelude::screen_width() {
            self.x = 0.
        }
        if self.y > macroquad::prelude::screen_height() {
            self.y = 0.
        }
        if self.y < 0. - self.size[1] {
            self.y = macroquad::prelude::screen_height()
        }
        if self.x < 0. - self.size[0] {
            self.x = macroquad::prelude::screen_width()
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

struct Obstacle {
    pos_list: Vec<[f32; 2]>,
    color: Color,
    is_hit: bool,
    size: [f32; 2],
}

impl Default for Obstacle {
    fn default() -> Self {
        Self {
            pos_list: Vec::new(),
            color: Color {
                r: 0.,
                g: 0.,
                b: 255.,
                a: 255.,
            },
            is_hit: false,
            size: [10., 10.],
        }
    }
}

impl Obstacle {
    pub fn generate_obstacles(object_count: i8) -> Obstacle {
        let mut thread_rng: ThreadRng = rand::thread_rng();
        let mut obs_list: Vec<[f32; 2]> = Vec::new();

        for _ in 0..=object_count {
            let y = thread_rng.gen_range(0.0..macroquad::prelude::screen_height());
            let x = thread_rng.gen_range(0.0..macroquad::prelude::screen_width());

            obs_list.push([x, y])
        }

        return Obstacle {
            pos_list: obs_list,
            ..Default::default()
        };
    }

    pub fn exist(&mut self, snake: Snake, food: Food) -> &mut Obstacle {

        for pos in &self.pos_list {
            macroquad::shapes::draw_rectangle(
                pos[0],
                pos[1],
                self.size[0],
                self.size[1],
                self.color,
            );
        }

        let tolerance = -self.size[0] * food.speed_boost..=self.size[1] * food.speed_boost;

        if self
            .pos_list
            .iter()
            .any(|f| tolerance.contains(&(f[0] - snake.positions.iter().next().unwrap()[0])) && tolerance.contains(&(f[1] - snake.positions.iter().next().unwrap()[1])) )
        {
            self.is_hit = true;  
        } else {
            self.is_hit = false;
        }

        self
    }
}

async fn redraw() {
    macroquad::prelude::next_frame().await;
}

async fn draw_ui(food_struct: &Food) {
    macroquad::text::draw_text(
        &(food_struct.score - 1).to_string(),
        100.,
        100.,
        50.,
        macroquad::color::WHITE,
    );
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

    let mut obstacle: Obstacle = Obstacle::generate_obstacles(20);

    let mut snake_struct = Snake::default();

    let mut food_struct = Food::default();

    loop {
        draw_ui(&food_struct).await;

        food_struct.snake = snake_struct.clone();

        //redraw screen
        redraw().await;

        food_struct.food();

        snake_struct.snake(food_struct.score.into(), food_struct.speed_boost);

        obstacle.exist(snake_struct.clone(), food_struct.clone());

        if !snake_struct.is_alive || obstacle.is_hit {
            break;
        }
    }
}
