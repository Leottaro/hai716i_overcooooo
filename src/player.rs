use crate::objets::{Direction, Ingredient};

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    position: (usize, usize),
    object_held: Option<Ingredient>,
    facing: Direction,
}

impl Player {
    pub fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            object_held: None,
            facing: Direction::North,
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        self.position
    }

    pub fn set_pos(&mut self, x: usize, y: usize, direction: Direction) {
        self.position = (x, y);
        self.facing = direction;
    }

    pub fn get_facing(&self) -> Direction {
        self.facing
    }

    pub fn set_facing(&mut self, direction: Direction) {
        self.facing = direction;
    }

    pub fn get_object_held(&self) -> Option<Ingredient> {
        self.object_held
    }

    pub fn take_object_held(&mut self) -> Option<Ingredient> {
        self.object_held.take()
    }

    pub fn set_object_held(&mut self, object: Option<Ingredient>){
        self.object_held = object
    }
}