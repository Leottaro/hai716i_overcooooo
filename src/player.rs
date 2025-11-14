use crate::objets::{Assiette, Direction, Ingredient};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerHand {
    Ingredient(Ingredient),
    Assiette(Assiette),
    Nothing,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    position: (usize, usize),
    object_held: PlayerHand,
    facing: Direction,
    blocked: bool,
}

impl Player {
    pub fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            object_held: PlayerHand::Nothing,
            facing: Direction::North,
            blocked: false,
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

    pub fn get_object_held(&self) -> PlayerHand {
        self.object_held.clone()
    }

    pub fn take_object_held(&mut self) -> PlayerHand {
        let object = self.object_held.clone();
        self.object_held = PlayerHand::Nothing;
        return object;
    }

    pub fn set_object_held(&mut self, object: PlayerHand) {
        self.object_held = object
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    pub fn block(&mut self) {
        self.blocked = true;
    }
    pub fn unblock(&mut self) {
        self.blocked = false;
    }
}
