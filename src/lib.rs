use std::{ops::RangeInclusive, time::Duration};

pub mod app;
pub mod game;
pub mod objets;
pub mod player;

const APP_TITLE: &str = "Overcook TUI";

pub const RECETTE_COOLDOWN_RANGE: RangeInclusive<Duration> =
    Duration::from_secs(10)..=Duration::from_secs(25);
pub const fn recette_deadline_range(ingredient_count: usize) -> RangeInclusive<Duration> {
    let bonus = 5 * ingredient_count as u64;
    Duration::from_secs(7 + bonus)
        ..=Duration::from_secs(20 + bonus)
}

const ROBOT_COOLDOWN: Duration = Duration::from_millis(100);

const MAX_VIES: i32 = 100;
