use std::collections::HashSet;
use std::time::Instant;
use std::{fmt::Display, time::Duration};

use rand::Rng;
use rand::seq::SliceRandom;

use crate::recette_deadline_range;

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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    pub fn iter() -> Vec<Self> {
        vec![
            IngredientType::Pain,
            IngredientType::Salade,
            IngredientType::Tomate,
            IngredientType::Oignon,
        ]
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

    pub fn into_coupe(mut self) -> Self {
        self.couper();
        self
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
#[derive(Debug, PartialEq, Clone)]
pub struct Recette {
    pub ingredients: HashSet<Ingredient>,
    pub creation: Instant,
    pub duree: Duration,
    pub expiration: Instant,
}

impl Recette {
    pub fn new(creation: Instant) -> Self {
        let mut rng = rand::rng();
        let mut ingredients = vec![Ingredient::new(IngredientType::Pain).into_coupe()];
        let mut possibles = [
            Ingredient::new(IngredientType::Salade).into_coupe(),
            Ingredient::new(IngredientType::Tomate).into_coupe(),
            Ingredient::new(IngredientType::Oignon).into_coupe(),
        ];
        let n = rng.random_range(1..=possibles.len());
        possibles.shuffle(&mut rng);
        for choice in possibles.into_iter().take(n) {
            ingredients.push(choice);
        }

        let duree = rng.random_range(recette_deadline_range(ingredients.len()));
        let expiration = creation + duree;

        Self {
            ingredients: ingredients.into_iter().collect::<HashSet<_>>(),
            creation,
            duree,
            expiration,
        }
    }

    pub fn default_recipe() -> Self {
        Recette::new(Instant::now())
    }

    pub fn get_ingredients(&self) -> &HashSet<Ingredient> {
        &self.ingredients
    }

    pub fn get_creation(&self) -> &Instant {
        &self.creation
    }

    pub fn get_duree(&self) -> &Duration {
        &self.duree
    }

    pub fn get_expiration(&self) -> &Instant {
        &self.expiration
    }

    pub fn is_too_late(&self, now: Instant) -> bool {
        self.expiration <= now
    }

    pub fn get_temps_restant(&self) -> Duration {
        self.expiration - Instant::now()
    }

    pub fn get_percent_left(&self) -> f32 {
        self.get_temps_restant().as_secs_f32() / self.duree.as_secs_f32()
    }

    pub fn is_same(&self, other: &Recette) -> bool {
        self.ingredients.eq(&other.ingredients)
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
