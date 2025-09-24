use std::ops::RangeInclusive;
use std::{fmt::Display, time::Duration};
use std::time::Instant;

use rand::{Rng, seq::IndexedRandom};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn emoji(&self) -> &'static str {
        match self {
            Direction::North => "⬆️",
            Direction::West => "⬅️",
            Direction::South => "⬇️",
            Direction::East => "➡️",
        }
    }
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

    pub fn emoji(&self) -> &'static str {
        match self {
            IngredientType::Pain => "🥖",
            IngredientType::Salade => "🥬",
            IngredientType::Tomate => "🍅",
            IngredientType::Oignon => "🧅",
        }
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

    pub fn emoji(&self) -> &'static str {
        match (self.type_ingredient, self.etat) {
            (IngredientType::Pain, IngredientEtat::Normal) => "🥖",
            (IngredientType::Pain, IngredientEtat::Coupe) => "🍞",
            (IngredientType::Salade, IngredientEtat::Normal) => "🥬",
            (IngredientType::Salade, IngredientEtat::Coupe) => "🥗",
            (IngredientType::Tomate, IngredientEtat::Normal) => "🍅",
            (IngredientType::Tomate, IngredientEtat::Coupe) => "🍅",
            (IngredientType::Oignon, IngredientEtat::Normal) => "🧅",
            (IngredientType::Oignon, IngredientEtat::Coupe) => "🧅",
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Case {
    Vide,
    Table(Option<Ingredient>),
    Ingredient(IngredientType),
    COUPER,
    ASSIETTE,
}

pub const RECETTE_DEADLINE_RANGE: RangeInclusive<Duration> = Duration::from_secs(30)..=Duration::from_secs(40);

#[derive(Debug, PartialEq, Clone)]
pub struct Recette {
    ingredients: Vec<Ingredient>,
    creation: Instant,
    duree: Duration,
    expiration: Instant,
}

impl Recette {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut ingredients = vec![Ingredient::new(IngredientType::Pain)];
        let possibles = [
            Ingredient::new(IngredientType::Salade),
            Ingredient::new(IngredientType::Tomate),
            Ingredient::new(IngredientType::Oignon),
        ];

        if let Some(&choice) = possibles.choose(&mut rng) {
            ingredients.push(choice);
        }

        let creation = Instant::now();
        let duree = rng.random_range(RECETTE_DEADLINE_RANGE);
        let expiration = creation + duree;

        Self {
            ingredients,
            creation,
            duree,
            expiration,
        }
    }
    
    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }

    pub fn is_too_late(&self) -> bool {
        self.expiration <= Instant::now()
    }

    pub fn get_temps_initial(&self) -> &Duration {
        &self.duree
    }

    pub fn get_temps_restant(&self) -> Duration {
        self.expiration - Instant::now()
    }

    pub fn get_percent_left(&self) -> f32 {
        self.get_temps_restant().as_secs_f32() / self.duree.as_secs_f32()
    }
}

impl Display for Recette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ingredients = self
            .ingredients
            .iter()
            .map(Ingredient::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let temps = self.get_temps_restant().as_secs_f32();
        write!(f, "{temps:.2}s, [{ingredients}]")
    }
}

#[derive(Debug, PartialEq)]
pub enum PickupError {
    HandsFull,
    AssietteEmpty,
    TableEmpty,
    NoTarget(((usize, usize), Case)),
}

#[derive(Debug, PartialEq)]
pub enum DepositError {
    HandsEmpty,
    TableFull,
    NoTarget(((usize, usize), Case)),
}

#[derive(Debug)]
pub enum RobotAction {
    Deplacer(Direction),
    Pickup,
    Deposit,
    None,
}
