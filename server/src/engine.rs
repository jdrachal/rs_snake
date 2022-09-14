
pub mod common; 

pub mod game_controller {
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::io::ErrorKind;
pub use crate::engine::common::{remove_lowest_occurence, 
    frequency_map, is_all_same, oposite};
use std::collections::{HashMap, LinkedList};

const MAP_WIDTH: i32 = 16;
const FRUIT_SPAWN_CHANCE: u32 = 15;

#[derive(Clone, Copy, Eq, Serialize, Deserialize, PartialEq, Debug)]
pub struct Position {
    x: i32,
    y: i32
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SnakeState {
    direction: String,
    positions: LinkedList<Position>
}

#[derive(Clone, Serialize)]
pub struct GameState {
    pub running: bool,
    pub snake_state: SnakeState,
    pub fruit_positions: Vec<Position>,
    pub user_input: HashMap<String, u32>
}


impl GameState{
    pub fn new() -> GameState {
        GameState {
            running: false,
            snake_state: SnakeState{
                    direction: "right".into(),
                    positions: LinkedList::from([
                        Position{x: 1,y: 0},
                        Position{x: 1,y: 0},
                        Position{x: 2,y: 0},
                        Position{x: 3,y: 0}])
                },
            fruit_positions: Vec::new(),
            user_input: HashMap::from([
                ("up".into(), 0),
                ("down".into(), 0),
                ("left".into(), 0),
                ("right".into(), 0)])
        }
    }

    pub fn reset(&mut self) {
        self.snake_state.direction = "right".into();
        self.snake_state.positions = LinkedList::from([
            Position{x: 0,y: 0},
            Position{x: 1,y: 0},
            Position{x: 2,y: 0},
            Position{x: 3,y: 0}]);
        self.fruit_positions = Vec::new();
        self.user_input = HashMap::from([
            ("up".into(), 0),
            ("down".into(), 0),
            ("left".into(), 0),
            ("right".into(), 0)]);
    }

    pub fn update_fruits(&mut self){
        let mut rng = rand::thread_rng();
    
        if rng.gen_range(0..100) < FRUIT_SPAWN_CHANCE {
            loop {
                let new_fruit = Position{x: rng.gen_range(0..MAP_WIDTH), y: rng.gen_range(0..MAP_WIDTH)};
                if let Ok(()) = self.clone().verify_collision(new_fruit.clone()) {
                    self.fruit_positions.push(new_fruit);
                    self.fruit_positions.dedup();
                    
                    break;
                };
            }
        }
    }
    
    pub fn update_direction(&mut self) {
        let mut directions: Vec<String> = Vec::new();
        
        let mut rng = rand::thread_rng();
    
        let mut op = oposite(self.snake_state.direction.clone());
        *self.user_input.get_mut(&mut op).unwrap() = 0;
    
        for (k, v) in self.user_input.clone() {
            if v == 1 {
                directions.push(k);
            }
        }
    
        if directions.len() ==  0 {
            return
        }
    
        loop {
    
            if is_all_same(&directions) {
                break;
            }
            let temp_directions: Vec<String> = directions.clone();
            directions.clear();
            for _ in 0..5 {
                let rand = rng.gen_range(0..temp_directions.len());
                directions.push(temp_directions[rand].clone());
            }
            remove_lowest_occurence(&mut directions);
        }
        self.snake_state.direction = directions.first().unwrap().clone();
    }
    
    pub fn move_snake(&mut self) -> Result<(), std::io::Error>{
    
        let mut x = self.snake_state.positions.back().unwrap().x;
        let mut y = self.snake_state.positions.back().unwrap().y;
        let dir = self.snake_state.direction.clone();
        
        if dir == "up" {
            y -= 1;
        } else if dir == "down" {
            y += 1;
        } else if dir == "left" {
            x -= 1;
        } else if dir == "right" {
            x += 1;
        }
    
        if x >= MAP_WIDTH {
            x = 0;
        }
    
        if x < 0 {
            x = MAP_WIDTH - 1;
        }
    
        if y >= MAP_WIDTH {
            y = 0;
        }
    
        if y < 0 {
            y = MAP_WIDTH - 1;
        }
    
        let next_position = Position{x: x, y: y};
    
        if let Err(e) = self.clone().verify_collision(next_position) {
            return Err(e);
        }
    
        self.snake_state.positions.push_back(next_position);
        self.snake_state.positions.pop_front();
    
        if self.fruit_positions.contains(&next_position.clone()) {
            if let Err(e) = self.enlarge_snake(){
                return Err(e);
            }
            let index = self.fruit_positions.iter().position(|x| *x == next_position).unwrap();
            self.fruit_positions.remove(index);
        }
    
        Ok(())
    }

    fn enlarge_snake(&mut self) -> Result<(), std::io::Error> {
        let mut enlarged_position = Position{x: 0, y: 0};
        let mut pos = self.snake_state.positions.iter();
        let (last, last_by_one) = (pos.next().unwrap(), pos.next().unwrap());
        enlarged_position.x = last.x + (last.x - last_by_one.x);
        enlarged_position.y = last.y + (last.y - last_by_one.y);
    
        if enlarged_position.x > MAP_WIDTH {
            enlarged_position.x -= MAP_WIDTH;
        }
    
        if enlarged_position.x < 0 {
            enlarged_position.x += MAP_WIDTH;
        }
    
        if enlarged_position.y > MAP_WIDTH {
            enlarged_position.y -= MAP_WIDTH;
        }
    
        if enlarged_position.y < 0 {
            enlarged_position.y += MAP_WIDTH;
        }
    
        let result = match self.clone().verify_collision(enlarged_position.clone()) {
            Ok(_) => {
                self.snake_state.positions.push_front(enlarged_position.clone());
                Ok(())
            },
            Err(e) => Err(e)
        };
        
        result
    }

    fn verify_collision(self, pos: Position) -> Result<(), std::io::Error> {
        for s in self.snake_state.positions {
            if pos == s{
                return Result::Err(std::io::Error::new(ErrorKind::Unsupported, "Collision"))
            }
        }
    
        Ok(())
    }
}

}