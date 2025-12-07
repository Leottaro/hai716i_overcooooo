use crate::objets::{Assiette, Direction, Ingredient};
use rand::Rng;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerHand {
    Ingredient(Ingredient),
    Assiette((usize, Assiette)),
    Nothing,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayerRecipeStrategy {
    LatestExpiration,
    Closest,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayerIngredientStrategy {
    NbApparitionInRecipe,
    Nearest,
}

pub fn random_recipe_strategy() -> PlayerRecipeStrategy {
    let mut rng = rand::rng();
    let strategies = [
        PlayerRecipeStrategy::LatestExpiration,
        PlayerRecipeStrategy::Closest,
    ];
    strategies[rng.random_range(0..strategies.len())]
}

pub fn random_ingredient_strategy() -> PlayerIngredientStrategy {
    let mut rng = rand::rng();
    let strategies = [
        PlayerIngredientStrategy::NbApparitionInRecipe,
        PlayerIngredientStrategy::Nearest,
    ];
    strategies[rng.random_range(0..strategies.len())]
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    position: (usize, usize),
    recipes_strategy: PlayerRecipeStrategy,
    ingredients_strategy: PlayerIngredientStrategy,
    object_held: PlayerHand,
    facing: Direction,
    blocked: bool,
}

impl Player {
    pub fn new(
        position: (usize, usize),
        recipes_strategy: PlayerRecipeStrategy,
        objectives_strategy: PlayerIngredientStrategy,
    ) -> Self {
        Self {
            position,
            recipes_strategy,
            ingredients_strategy: objectives_strategy,
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

    pub fn get_recipes_strategy(&self) -> PlayerRecipeStrategy {
        self.recipes_strategy
    }
    pub fn get_ingredients_strategy(&self) -> PlayerIngredientStrategy {
        self.ingredients_strategy
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
        object
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
