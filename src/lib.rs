use std::{ops::RangeInclusive, time::Duration};

pub mod app;
pub mod game;
pub mod objets;
pub mod player;

const APP_TITLE: &str = "Overcook TUI";

pub const GAME_DURATION: Duration = Duration::from_secs(120);

pub const RECETTE_COOLDOWN_RANGE: RangeInclusive<Duration> =
    Duration::from_secs(10)..=Duration::from_secs(25);
pub const fn recette_deadline_range(ingredient_count: usize) -> RangeInclusive<Duration> {
    let bonus = 5 * ingredient_count as u64;
    let mallus = ingredient_count as u64 / 2;
    Duration::from_secs(7 + bonus - mallus)..=Duration::from_secs(20 + bonus - mallus)
}

const ROBOT_COOLDOWN: Duration = Duration::from_millis(100);
