use std::fmt::Display;

use rand::{seq::IndexedRandom, Rng};

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IngredientType {
    Pain,
    Salade,
    Tomate,
    Oignon,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IngredientEtat {
    Normal,
    Coupe,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ingredient {
    type_ingredient: IngredientType,
    etat: IngredientEtat,
}

impl Ingredient {
    pub fn new(type_ingredient: IngredientType) -> Self {
        Self {
            type_ingredient,
            etat: IngredientEtat::Normal,
        }
    }

    pub fn char(&self) -> char {
        match self.type_ingredient {
            IngredientType::Pain => 'p',
            IngredientType::Salade => 's',
            IngredientType::Tomate => 't',
            IngredientType::Oignon => 'o',
        }
    }

    pub fn upper_char(&self) -> char {
        self.char().to_uppercase().next().unwrap()
    }

    pub fn couper(&mut self) {
        self.etat = IngredientEtat::Coupe;
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Case {
    Vide,
    Table(Option<Ingredient>),
    Ingredient(Ingredient),
    COUPER,
    ASSIETTE,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recette {
    ingredients: Vec<Ingredient>,
    deadline: usize,
}

impl Recette{
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut ingredients = vec![Ingredient::new(IngredientType::Pain)]; // de base il y a du pain
        let possibles = vec![Ingredient::new(IngredientType::Salade), Ingredient::new(IngredientType::Tomate), Ingredient::new(IngredientType::Oignon)];

        if let Some(choice) = possibles.choose(&mut rng) {
            ingredients.push(choice.clone());
        }
        let deadline = rng.random_range(30*5..=40*5);
        
        Self {
            ingredients,
            deadline : deadline,
        }
    }

    pub fn pass_time(&mut self) {
        self.deadline -= 1;
    }

    pub fn is_too_late(&self) -> bool {
        self.deadline == 0
    }
}

impl Display for Recette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ingredients = self.ingredients
                    .iter()
                    .map(Ingredient::to_string)
                    .fold(String::new(), |i1, i2| format!("{i1},{i2}"));
        let temps = self.deadline;
        write!(f, "{temps}:{ingredients}")
    }
}