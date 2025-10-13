use crate::{
    objets::{Case, Direction, Ingredient, IngredientEtat, IngredientType, Recette},
    player::Player,
};
use std::{
    collections::HashSet,
    ops::RangeInclusive,
    time::{Duration, Instant},
};

const MAX_VIES: i32 = 100;

pub const RECETTE_COOLDOWN_RANGE: RangeInclusive<Duration> =
    Duration::from_secs(2)..=Duration::from_secs(3);

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

#[derive(Debug, PartialEq)]
pub struct Game {
    player: Player,
    assiette: Vec<Ingredient>,
    map: Vec<Vec<Case>>,
    recettes: Vec<Recette>,

    score: i32,
    vie: i32,
    next_recette: Instant,
}

impl Game {
    pub fn new() -> Self {
        let map: Vec<Vec<Case>> = vec![
            vec![
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::ASSIETTE,
                Case::Table(None),
            ],
            vec![
                Case::Ingredient(IngredientType::Pain),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::COUPER,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::COUPER,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::COUPER,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
                Case::Vide,
                Case::Vide,
                Case::Table(None),
            ],
            vec![
                Case::Table(None),
                Case::Ingredient(IngredientType::Tomate),
                Case::Table(None),
                Case::Ingredient(IngredientType::Salade),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Table(None),
                Case::Ingredient(IngredientType::Oignon),
                Case::Table(None),
                Case::Table(None),
            ],
        ];

        Self {
            player: Player::new((1, 1)),
            map,
            recettes: vec![Recette::new()],
            assiette: Vec::new(),
            score: 0,
            vie: MAX_VIES,
            next_recette: Instant::now() + rand::random_range(RECETTE_COOLDOWN_RANGE),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_assiette(&self) -> &Vec<Ingredient> {
        &self.assiette
    }

    pub fn get_recettes(&self) -> &Vec<Recette> {
        &self.recettes
    }

    pub fn get_map(&self) -> &Vec<Vec<Case>> {
        &self.map
    }

    pub fn get_map_heigth(&self) -> usize {
        self.map.len()
    }

    pub fn get_map_width(&self) -> usize {
        self.map[0].len()
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_vies(&self) -> i32 {
        self.vie
    }

    pub fn get_facing(&self, pos: (usize, usize)) -> ((usize, usize), Case) {
        let mut facing_pos: (usize, usize) = pos;
        let lenx: usize = self.map[0].len();
        let leny: usize = self.map.len();

        match self.player.get_facing() {
            Direction::North => facing_pos.1 = pos.1 - 1,
            Direction::West => facing_pos.0 = pos.0 - 1,
            Direction::South => facing_pos.1 = pos.1 + 1,
            Direction::East => facing_pos.0 = pos.0 + 1,
        }

        if facing_pos.0 >= lenx || facing_pos.1 >= leny {
            return (facing_pos, Case::Vide);
        }

        (facing_pos, self.map[facing_pos.1][facing_pos.0])
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < self.map[0].len() - 1 {
            neighbours.push((x + 1, y));
        }
        if y < self.map.len() - 1 {
            neighbours.push((x, y + 1));
        }
        neighbours
    }

    pub fn add_random_recette(&mut self) {
        self.recettes.push(Recette::new());
        self.recettes
            .sort_by(|r1, r2| r1.get_expiration().cmp(r2.get_expiration()));
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.set_facing(direction);
        let wanted_pos: (usize, usize) = self.get_facing(self.player.get_pos()).0;
        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide {
            self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
    }

    pub fn pickup(&mut self) -> Result<(), PickupError> {
        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        if self.player.get_object_held().is_some() {
            return Err(PickupError::HandsFull);
        }

        match facing_object {
            Case::ASSIETTE => {
                if let Some(ingredient) = self.assiette.pop() {
                    self.player.set_object_held(Some(ingredient));
                } else {
                    return Err(PickupError::AssietteEmpty);
                }
            }
            Case::Ingredient(object) => self.player.set_object_held(Some(Ingredient::new(object))),
            Case::Table(None) => return Err(PickupError::TableEmpty),
            Case::Table(ingredient) => {
                self.player.set_object_held(ingredient);
                self.map[facing_pos.1][facing_pos.0] = Case::Table(None);
            }
            _ => return Err(PickupError::NoTarget((facing_pos, facing_object))),
        }

        Ok(())
    }

    pub fn deposit(&mut self) -> Result<(), DepositError> {
        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        let object_held = match self.player.take_object_held() {
            None => return Err(DepositError::HandsEmpty),
            Some(obj) => obj,
        };

        match facing_object {
            Case::ASSIETTE => {
                self.assiette.push(object_held);
            }
            Case::Table(None) => {
                self.map[facing_pos.1][facing_pos.0] = Case::Table(Some(object_held));
            }
            Case::Table(_) => return Err(DepositError::TableFull),
            Case::COUPER => {
                let mut ingredient = object_held;
                ingredient.couper();
                self.player.set_object_held(Some(ingredient));
            }
            _ => return Err(DepositError::NoTarget((facing_pos, facing_object))),
        }

        Ok(())
    }

    pub fn tick(&mut self, now: Instant) {
        let (recettes_too_late, mut new_recettes): (Vec<_>, Vec<_>) = self
            .recettes
            .clone()
            .into_iter()
            .partition::<Vec<_>, _>(|recette| recette.is_too_late(now));

        let assiette_hashset = self.assiette.clone().into_iter().collect::<HashSet<_>>();
        let recette_correspondante = self
            .recettes
            .iter()
            .position(|recette| assiette_hashset.eq(recette.get_ingredients()));
        if let Some(i) = recette_correspondante {
            let bonus = self.assiette.len() as i32;
            self.score += bonus;
            self.vie = if self.vie > MAX_VIES - bonus * 5 {
                MAX_VIES
            } else {
                self.vie + bonus * 5
            };
            self.assiette.clear();
            new_recettes.remove(i);
        }

        for recette in &recettes_too_late {
            let mallus = (recette.get_ingredients().len() as i32) * 10;
            self.vie -= if self.vie < mallus { self.vie } else { mallus };
        }

        // update the too lates recettes
        self.recettes = new_recettes;
        if self.next_recette <= now {
            self.recettes.push(Recette::new());
            self.next_recette = now + rand::random_range(RECETTE_COOLDOWN_RANGE)
                - Duration::from_secs((self.score as u64) / 10);
        }
    }

    pub fn robot(&mut self) {
        let action = self.determine_action();
        match action {
            RobotAction::Deplacer(direction) => self.move_player(direction),
            RobotAction::Pickup => self.pickup().expect("Failed to pick up ingredient"),
            RobotAction::Deposit => self.deposit().expect("Failed to deposit ingredient"),
            RobotAction::None => (),
        }
    }

    fn determine_action(&self) -> RobotAction {
        let objectives = self.determine_objectives();

        let (x, y) = self.player.get_pos();
        for objective in objectives {
            let chemin = match self.pathfind_case((x, y), objective) {
                None => continue,
                Some(chemin) => chemin,
            };

            let next_pos = match chemin.get(1) {
                Some(value) => value.clone(),
                None => continue,
            };

            let direction = match next_pos {
                (x1, y1) if (x1, y1) == (x, y - 1) => Direction::North,
                (x1, y1) if (x1, y1) == (x, y + 1) => Direction::South,
                (x1, y1) if (x1, y1) == (x - 1, y) => Direction::West,
                (x1, y1) if (x1, y1) == (x + 1, y) => Direction::East,
                _ => continue,
            };

            if chemin.len() != 2 || self.player.get_facing() != direction {
                return RobotAction::Deplacer(direction);
            }

            if self.player.get_object_held().is_none() {
                return RobotAction::Pickup;
            } else {
                return RobotAction::Deposit;
            }
        }

        RobotAction::None
    }

    fn determine_objectives(&self) -> Vec<Case> {
        let assiette_hashset = self.assiette.clone().into_iter().collect::<HashSet<_>>();

        let mut diff = usize::MAX;
        let mut assiette_priv_recette: HashSet<Ingredient> = HashSet::new();
        let mut recette_priv_assiette: HashSet<Ingredient> = HashSet::new();

        for recette in self.recettes.iter() {
            let current_recette_priv_assiette = recette
                .get_ingredients()
                .difference(&assiette_hashset)
                .cloned()
                .collect::<HashSet<_>>();
            let current_assiette_priv_recette = assiette_hashset
                .difference(recette.get_ingredients())
                .cloned()
                .collect::<HashSet<_>>();
            let current_diff =
                current_assiette_priv_recette.len() + current_recette_priv_assiette.len();
            if current_diff < diff {
                diff = current_diff;
                assiette_priv_recette = current_assiette_priv_recette;
                recette_priv_assiette = current_recette_priv_assiette;
            }
        }

        if diff == usize::MAX {
            return vec![];
        }

        // TODO: objectives have different imortance level, in a same level take the nearest objective
        if !assiette_priv_recette.is_empty() {
            if let Some(held_ingredient) = self.player.get_object_held() {
                if !assiette_priv_recette.contains(&held_ingredient) {
                    return vec![Case::Table(None)];
                }
            }
            return vec![Case::ASSIETTE];
        } else if recette_priv_assiette.is_empty() {
            panic!("assiette = next recette mais on est pas passé à la suite ??? :\n{self:#?}");
        }

        if let Some(held_ingredient) = self.player.get_object_held() {
            if recette_priv_assiette.contains(&held_ingredient) {
                return vec![Case::ASSIETTE];
            } else if recette_priv_assiette
                .iter()
                .find(|ingr| held_ingredient.type_ingredient.eq(&ingr.type_ingredient))
                .is_some()
            {
                return vec![Case::COUPER];
            } else {
                return vec![Case::Table(None)];
            }
        }

        let mut recette_priv_assiette_vec = recette_priv_assiette.into_iter().collect::<Vec<_>>();
        // TODO: choisir l'ingredient qui apparait dans les recettes d'apres (au cas où la recette actuelle se termine)
        recette_priv_assiette_vec.sort();
        let next_ingredient = recette_priv_assiette_vec[0];

        vec![
            Case::Table(Some(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Coupe,
            })),
            Case::Table(Some(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Normal,
            })),
            Case::Ingredient(next_ingredient.type_ingredient),
        ]
    }

    fn pathfind_case(&self, start: (usize, usize), case: Case) -> Option<Vec<(usize, usize)>> {
        let mut weights: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        let mut explored_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut next_positions: Vec<(usize, usize)> = vec![start];
        weights[start.1][start.0] = 0;

        let mut found_pos: Option<(usize, usize)> = None;
        while let Some((x, y)) = next_positions.pop() {
            if !explored_positions.insert((x, y)) {
                continue;
            }

            let mut min_neighbour = usize::MAX;
            for (x1, y1) in self.get_neighbours(x, y) {
                if weights[y1][x1] == usize::MAX {
                    if self.map[y1][x1] == Case::Vide {
                        next_positions.insert(0, (x1, y1));
                    } else if self.map[y1][x1] == case {
                        found_pos = Some((x1, y1));
                    }
                } else if weights[y1][x1] < min_neighbour {
                    min_neighbour = weights[y1][x1];
                }
            }

            if min_neighbour != usize::MAX {
                weights[y][x] = min_neighbour + 1;
            }

            if found_pos.is_some() {
                break;
            }
        }

        found_pos?;
        let mut path = vec![found_pos.unwrap()];

        loop {
            let (x, y) = match path.first().cloned() {
                None => break,
                Some(pos) => {
                    if pos == start {
                        break;
                    } else {
                        pos
                    }
                }
            };

            let mut min_x = 0;
            let mut min_y = 0;
            let mut min_val = usize::MAX;
            for (x1, y1) in self.get_neighbours(x, y) {
                if weights[y1][x1] < min_val {
                    min_x = x1;
                    min_y = y1;
                    min_val = weights[y1][x1];
                }
            }
            if min_val != usize::MAX {
                path.insert(0, (min_x, min_y));
            }
        }

        Some(path)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            let line = row
                .iter()
                .enumerate()
                .map(|(x, case)| match case {
                    Case::Vide => {
                        if (x, y) == self.player.get_pos() {
                            "·".to_string()
                        } else {
                            " ".to_string()
                        }
                    }
                    Case::Table(None) => "#".to_string(),
                    Case::Table(Some(ingredient)) => ingredient.type_ingredient.char().to_string(),
                    Case::Ingredient(ingredient_type) => ingredient_type.upper_char().to_string(),
                    Case::COUPER => "C".to_string(),
                    Case::ASSIETTE => "O".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" ");

            writeln!(f, "{line}")?;
        }
        writeln!(f)?;

        let line = self
            .recettes
            .iter()
            .map(Recette::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        writeln!(f, "Recettes voulues : {}", line)?;

        let line = self
            .assiette
            .iter()
            .map(Ingredient::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, "Assiette : {line}")?;

        Ok(())
    }
}
