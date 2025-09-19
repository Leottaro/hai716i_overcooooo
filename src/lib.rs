use std::ops::RangeInclusive;

pub mod game;
pub mod objets;
pub mod player;
pub mod app;

pub const DEADLINE_RANGE: RangeInclusive<usize> = 30*5..=40*5;
pub const RECETTE_RANGE: RangeInclusive<usize> = 25..=50;
