use std::fmt::Display;

use rand::{seq::IndexedRandom, Rng};

use crate::DEADLINE_RANGE;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum IngredientType {
    Pain,
    Salade,
    Tomate,
    Oignon,
}

impl IngredientType {
    pub fn char(&self) -> char {
        match self {
            IngredientType::Pain => 'p',
            IngredientType::Salade => 's',
            IngredientType::Tomate => 't',
            IngredientType::Oignon => 'o',
        }
    }

    pub fn upper_char(&self) -> char {
        self.char().to_uppercase().next().unwrap()
    }
}

impl Display for IngredientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            IngredientType::Pain => "Pain",
            IngredientType::Salade => "Salade",
            IngredientType::Tomate => "Tomate",
            IngredientType::Oignon => "Oignon",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum IngredientEtat {
    Normal,
    Coupe,
}

impl Display for IngredientEtat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            IngredientEtat::Normal => "Normal",
            IngredientEtat::Coupe => "Coupé",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Ingredient {
    pub type_ingredient: IngredientType,
    pub etat: IngredientEtat,
}

impl Ingredient {
    pub fn new(type_ingredient: IngredientType) -> Self {
        Self {
            type_ingredient,
            etat: IngredientEtat::Normal,
        }
    }

    pub fn couper(&mut self) {
        self.etat = IngredientEtat::Coupe;
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.type_ingredient, self.etat)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recette {
    ingredients: Vec<Ingredient>,
    deadline: usize,
    temps_initial: usize,
}

impl Recette{
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut ingredients = vec![Ingredient::new(IngredientType::Pain)]; // de base il y a du pain
        let possibles = vec![Ingredient::new(IngredientType::Salade), Ingredient::new(IngredientType::Tomate), Ingredient::new(IngredientType::Oignon)];

        if let Some(choice) = possibles.choose(&mut rng) {
            ingredients.push(choice.clone());
        }
        let deadline = rng.random_range(DEADLINE_RANGE);
        let temps_initial = deadline;
        
        Self {
            ingredients,
            deadline : deadline,
            temps_initial
        }
    }

    pub fn pass_time(&mut self) {
        self.deadline -= 1;
    }

    pub fn is_too_late(&self) -> bool {
        self.deadline == 0
    }

    pub fn get_temps_init(&self) -> usize {
        self.temps_initial
    }

    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }

    pub fn get_deadline(&self) -> usize {
        self.deadline
    }
}

impl Display for Recette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ingredients = self.ingredients
                    .iter()
                    .map(Ingredient::to_string)
                    .collect::<Vec<_>>().join(", ");
        let temps = self.deadline;
        write!(f, "{temps}t, [{ingredients}]")
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Case {
    Vide,
    Table(Option<Ingredient>),
    Ingredient(IngredientType),
    COUPER,
    ASSIETTE,
}

#[derive(Debug)]
pub enum RobotAction {
    Deplacer(Direction),
    Pickup,
    Deposit,
    None,
}