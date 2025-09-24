use std::fmt::Display;
use std::time::Instant;

use rand::{Rng, seq::IndexedRandom};

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

    pub fn char(&self) -> char {
        match self.type_ingredient {
            IngredientType::Pain => 'p',
            IngredientType::Salade => 's',
            IngredientType::Tomate => 't',
            IngredientType::Oignon => 'o',
        }
    }

    pub fn emoji(&self) -> &'static str {
        match (self.type_ingredient, self.etat) {
            (IngredientType::Pain, IngredientEtat::Normal) => "🥖",
            (IngredientType::Pain, IngredientEtat::Coupe) => "🍞",
            (IngredientType::Salade, IngredientEtat::Normal) => "🥬",
            (IngredientType::Salade, IngredientEtat::Coupe) => "🥗",
            (IngredientType::Tomate, IngredientEtat::Normal) => "🍅",
            (IngredientType::Tomate, IngredientEtat::Coupe) => "🍅", // Même emoji pour coupé
            (IngredientType::Oignon, IngredientEtat::Normal) => "🧅",
            (IngredientType::Oignon, IngredientEtat::Coupe) => "🧅", // Même emoji pour coupé
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
        write!(f, "{} {}", self.type_ingredient, self.etat)
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionResult {
    Success,
    Blocked,         // Chemin bloqué
    HandsFull,       // Mains pleines
    HandsEmpty,      // Mains vides
    NoTarget,        // Rien à interagir
    InvalidPosition, // Position invalide
    TableOccupied,   // Table déjà occupée
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
    temps_restant: u32,
    temps_initial: u32,
    last_update: Instant,
}

impl Recette {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut ingredients = vec![Ingredient::new(IngredientType::Pain)]; // de base il y a du pain
        let possibles = [
            Ingredient::new(IngredientType::Salade),
            Ingredient::new(IngredientType::Tomate),
            Ingredient::new(IngredientType::Oignon),
        ];

        if let Some(&choice) = possibles.choose(&mut rng) {
            ingredients.push(choice);
        }
        let temps_initial = rng.random_range(DEADLINE_RANGE.clone()) as u32;
        let temps_restant = temps_initial;

        Self {
            ingredients,
            temps_initial,
            temps_restant,
            last_update: Instant::now(),
        }
    }

pub fn update(&mut self) {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_update).as_secs() as u32;
    if elapsed > 0 {
        self.temps_restant = self.temps_restant.saturating_sub(elapsed);
        self.last_update = now;
    }
}

pub fn pass_time(&mut self) {
    self.temps_restant -= 1;
}

pub fn is_too_late(&self) -> bool {
    self.temps_restant == 0
}

pub fn get_ingredients(&self) -> &Vec<Ingredient> {
    &self.ingredients
}

pub fn get_temps_initial(&self) -> u32 {
    self.temps_initial
}

pub fn get_temps_restant(&self) -> u32 {
    self.temps_restant
}

}

impl Default for Recette {
    fn default() -> Self {
        Recette {
            ingredients: vec![Ingredient::new(IngredientType::Pain)],
            temps_initial: *DEADLINE_RANGE.start() as u32,
            temps_restant: *DEADLINE_RANGE.start() as u32,
            last_update: Instant::now(),
        }
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
