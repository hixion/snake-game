use rand::prelude::*;
use std::ops::Add;

pub const GRID_X_SIZE: u32 = 40;
pub const GRID_Y_SIZE: u32 = 30;
pub const DOT_SIZE_IN_PXS: u32 = 20;

pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            food: Point(3, 3),
            state: GameState::Playing,
        }
    }

    pub fn next_tick(&mut self) {
        match self.state {
            GameState::Playing => self.update_game(),
            _ => return,
        }
    }

    fn update_game(&mut self) {
        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Left => *head_position + Point(-1, 0),
            PlayerDirection::Right => *head_position + Point(1, 0),
        };

        self.check_position(&next_head_position);

        self.player_position.pop();
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    fn check_position(&mut self, position: &Point) {
        if position.0 >= GRID_X_SIZE as i32
            || position.0 < 0
            || position.1 >= GRID_Y_SIZE as i32
            || position.1 < 0
        {
            self.state = GameState::GameOver;
        }
    }

    pub fn move_up(&mut self) {
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_left(&mut self) {
        self.player_direction = PlayerDirection::Left;
    }

    pub fn move_right(&mut self) {
        self.player_direction = PlayerDirection::Right;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
            _ => GameState::GameOver,
        }
    }

    pub fn feed(&mut self) {
        if *self.player_position.first().unwrap() == self.food {
            self.player_position.reverse();
            self.player_position.push(self.food);
            self.player_position.reverse();

            let mut rng = rand::thread_rng();
            let rand_pos: i32 = rng.gen_range(0..30);

            self.food.0 = (self.food.0 + rand_pos) % (GRID_X_SIZE) as i32;
            self.food.1 = (self.food.1 + rand_pos) % (GRID_Y_SIZE) as i32;
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
