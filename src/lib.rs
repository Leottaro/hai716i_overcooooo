use std::ops::RangeInclusive;

pub mod app;
pub mod game;
pub mod objets;
pub mod player;

const COEFF: usize = 1;
pub const DEADLINE_RANGE: RangeInclusive<usize> = 30 * COEFF..=40 * COEFF;
pub const RECETTE_RANGE: RangeInclusive<usize> = 25..=50;
