use crate::{
    BRULER_DURATION, COUPER_DURATION, CUIRE_DURATION, GAME_DURATION, RECETTE_COOLDOWN_RANGE,
    objets::{
        Assiette, Case, Direction, Ingredient, IngredientCuisson, IngredientEtat, IngredientType,
        Recette,
    },
    player::{Player, PlayerHand},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    process::exit,
    time::{Duration, Instant},
};

use csv::ReaderBuilder;

#[derive(Debug, PartialEq)]
pub enum PickupError {
    HandsFull,
    DepotEmpty,
    TableEmpty,
    NoTarget(((usize, usize), Case)),
}

#[derive(Debug, PartialEq)]
pub enum DepositError {
    HandsEmpty,
    TableFull,
    NoTarget(((usize, usize), Case)),
    NonCuisableIngredient,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RobotAction {
    Deplacer(Direction),
    Pickup,
    Deposit,
    None,
}

impl Display for RobotAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RobotAction::Deplacer(direction) => write!(f, "{}", direction.emoji()),
            RobotAction::Pickup => write!(f, "P"),
            RobotAction::Deposit => write!(f, "D"),
            RobotAction::None => write!(f, " "),
        }
    }
}

pub type CaseEvent = Box<dyn Fn(Instant) -> Option<Ingredient>>;

pub struct Game {
    player: Player,
    map: Vec<Vec<Case>>,
    recettes: Vec<Recette>,

    score: i32,
    next_recette: Instant,
    end_instant: Instant,
    is_finished: bool,
    cuire_events: HashMap<(usize, usize), CaseEvent>,
}

impl Game {
    fn lecture_map(file: String) -> Vec<Vec<Case>> {
        let height = 10;
        let width = 15;
        // Build the CSV reader and iterate over each record.
        let mut rdr = ReaderBuilder::new()
            .from_path(file)
            .expect("fichier csv pas trouve");
        let mut map: Vec<Vec<Case>> = vec![vec![Case::Vide; width]; height];
        let mut i = 0;
        for result in rdr.records() {
            let record = result.expect("probleme lecture csv");

            let mut j = 0;
            for r in record.into_iter() {
                let r = r.replace(" ", "").replace("\t", "");
                match r.as_str() {
                    "" => {
                        map[i][j] = Case::Vide;
                    }
                    "T" => {
                        map[i][j] = Case::Table(None);
                    }
                    "C" => {
                        map[i][j] = Case::Couper(None);
                    }
                    "F" => {
                        map[i][j] = Case::Cuire(None);
                    }
                    "A" => {
                        map[i][j] = Case::Assiette;
                    }
                    "D" => {
                        map[i][j] = Case::Depot(None);
                    }
                    s if s.starts_with("I(") && s.ends_with(")") => {
                        // I(ingredient)
                        let inner = &s[2..s.len() - 1];
                        match inner {
                            "T" => {
                                map[i][j] = Case::Ingredient(IngredientType::Tomate);
                            }
                            "O" => {
                                map[i][j] = Case::Ingredient(IngredientType::Oignon);
                            }
                            "S" => {
                                map[i][j] = Case::Ingredient(IngredientType::Salade);
                            }
                            "P" => {
                                map[i][j] = Case::Ingredient(IngredientType::Pain);
                            }
                            "C" => {
                                map[i][j] = Case::Ingredient(IngredientType::Poulet);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            i += 1;
        }
        map
    }

    pub fn new(file: String) -> Self {
        let map: Vec<Vec<Case>> = Game::lecture_map(file);

        Self {
            player: Player::new((1, 1)),
            map,
            recettes: vec![Recette::default_recipe()],
            score: 0,
            next_recette: Instant::now() + rand::random_range(RECETTE_COOLDOWN_RANGE),
            end_instant: Instant::now() + GAME_DURATION,
            is_finished: false,
            cuire_events: HashMap::new(),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
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

    pub fn get_end_instant(&self) -> &Instant {
        &self.end_instant
    }

    pub fn get_remaining_time(&self) -> Duration {
        self.end_instant - Instant::now()
    }

    pub fn get_percent_left(&self) -> f32 {
        self.get_remaining_time().as_secs_f32() / GAME_DURATION.as_secs_f32()
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn get_facing(&self, pos: (usize, usize)) -> ((usize, usize), &Case) {
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
            return (facing_pos, &Case::Vide);
        }

        (facing_pos, &self.map[facing_pos.1][facing_pos.0])
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

    fn add_random_recette(&mut self, now: Instant) {
        self.recettes.push(Recette::new(now));
        self.recettes.sort_by_key(|r| *r.get_expiration());
    }

    fn move_player(&mut self, direction: Direction) {
        if self.player.is_blocked() {
            return;
        }
        self.player.set_facing(direction);
        let wanted_pos: (usize, usize) = self.get_facing(self.player.get_pos()).0;
        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide {
            self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
    }

    pub fn pickup(&mut self) -> Result<(), PickupError> {
        if self.player.is_blocked() || self.is_finished {
            return Ok(());
        }

        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        if self.player.get_object_held() != PlayerHand::Nothing {
            return Err(PickupError::HandsFull);
        }

        match facing_object {
            Case::Ingredient(object) => self
                .player
                .set_object_held(PlayerHand::Ingredient(Ingredient::new(*object))),
            Case::Table(Some(content)) => {
                self.player.set_object_held(content.clone());
                self.map[facing_pos.1][facing_pos.0] = Case::Table(None);
            }
            Case::Table(None) => return Err(PickupError::TableEmpty),
            Case::Assiette => self
                .player
                .set_object_held(PlayerHand::Assiette(Assiette::new())),
            Case::Cuire(Some(ingr)) => {
                self.player.set_object_held(PlayerHand::Ingredient(*ingr));
                self.map[facing_pos.1][facing_pos.0] = Case::Cuire(None);
            }
            _ => return Err(PickupError::NoTarget((facing_pos, facing_object.clone()))),
        }

        Ok(())
    }

    pub fn deposit(&mut self, now: Instant) -> Result<(), DepositError> {
        if self.player.is_blocked() || self.is_finished {
            return Ok(());
        }

        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        match (self.player.get_object_held(), facing_object) {
            (hand, Case::Table(None)) => {
                self.map[facing_pos.1][facing_pos.0] = Case::Table(Some(hand));
                self.player.set_object_held(PlayerHand::Nothing);
            }
            (PlayerHand::Ingredient(ingr), Case::Table(Some(PlayerHand::Assiette(assiette)))) => {
                let mut new_assiette = assiette.clone();
                new_assiette.ingredients.push(ingr);
                self.map[facing_pos.1][facing_pos.0] =
                    Case::Table(Some(PlayerHand::Assiette(new_assiette)));
                self.player.set_object_held(PlayerHand::Nothing);
            }
            (_hand, Case::Table(Some(_))) => {
                return Err(DepositError::TableFull);
            }
            (PlayerHand::Ingredient(ingredient), Case::Couper(None)) => {
                self.player.set_object_held(PlayerHand::Nothing);
                self.player.block();
                self.map[facing_pos.1][facing_pos.0] = Case::Couper(Some(0)); // TODO: player_id
                self.cuire_events.insert(
                    facing_pos,
                    Box::new(move |instant| {
                        if instant > now + COUPER_DURATION {
                            Some(ingredient.into_coupe())
                        } else {
                            None
                        }
                    }),
                );
            }
            (PlayerHand::Ingredient(ingredient), Case::Cuire(None)) => {
                if !ingredient.cuisable {
                    return Err(DepositError::NonCuisableIngredient);
                }
                self.player.set_object_held(PlayerHand::Nothing);
                self.cuire_events.insert(
                    facing_pos,
                    Box::new(move |instant| {
                        if instant < now + CUIRE_DURATION {
                            Some(ingredient)
                        } else if instant < now + BRULER_DURATION {
                            Some(ingredient.into_cuit())
                        } else {
                            Some(ingredient.into_brule())
                        }
                    }),
                );
                self.map[facing_pos.1][facing_pos.0] = Case::Cuire(Some(ingredient));
            }
            (PlayerHand::Assiette(assiette), Case::Depot(None)) => {
                self.map[facing_pos.1][facing_pos.0] = Case::Depot(Some(assiette));
                self.player.set_object_held(PlayerHand::Nothing);
            }
            (PlayerHand::Ingredient(ingredient), Case::Assiette) => {
                self.player
                    .set_object_held(PlayerHand::Assiette(Assiette::create_with(ingredient)));
            }
            (PlayerHand::Nothing, Case::Assiette) => {
                self.player
                    .set_object_held(PlayerHand::Assiette(Assiette::new()));
            }
            (PlayerHand::Nothing, _) => return Ok(()),
            _ => return Err(DepositError::NoTarget((facing_pos, facing_object.clone()))),
        }

        Ok(())
    }

    pub fn tick(&mut self, now: Instant) {
        if self.is_finished {
            return;
        }

        // Update the too lates recettes
        let (recettes_too_late, new_recettes): (Vec<_>, Vec<_>) = self
            .recettes
            .clone()
            .into_iter()
            .partition::<Vec<_>, _>(|recette| recette.is_too_late(now));

        for recette in &recettes_too_late {
            self.score -= (IngredientType::iter().len() - recette.get_ingredients().len()) as i32
        }

        self.recettes = new_recettes;
        if self.next_recette <= now || self.recettes.len() < 2 {
            self.add_random_recette(now);
            if self.next_recette <= now {
                self.next_recette = now + rand::random_range(RECETTE_COOLDOWN_RANGE);
            }
        }

        // Update the Case
        for (y, line) in self.map.iter_mut().enumerate() {
            for (x, case) in line.iter_mut().enumerate() {
                match case {
                    Case::Couper(Some(player_id)) => {
                        if let Some(ingredient) =
                            self.cuire_events.get(&(x, y)).and_then(|func| func(now))
                        {
                            self.player
                                .set_object_held(PlayerHand::Ingredient(ingredient)); // TODO: player_id
                            self.player.unblock();
                            *case = Case::Couper(None);
                        }
                    }
                    Case::Cuire(Some(ingredient)) => {
                        if let Some(new_ingredient) =
                            self.cuire_events.get(&(x, y)).and_then(|func| func(now))
                        {
                            *ingredient = new_ingredient
                        }
                    }
                    Case::Depot(Some(assiette)) => {
                        let assiette_hashset = assiette.get_hashset();
                        let recette_correspondante = self
                            .recettes
                            .iter()
                            .position(|recette| assiette_hashset.eq(recette.get_ingredients()));
                        if let Some(i) = recette_correspondante {
                            let bonus = assiette.ingredients.len() as i32 * 2;
                            self.score += bonus;
                            self.recettes.remove(i);
                            *case = Case::Depot(None);
                        }
                    }
                    _ => (),
                }
            }
        }

        if self.end_instant <= now {
            self.is_finished = true;
        }
    }

    pub fn robot(&mut self, now: Instant) {
        if self.is_finished {
            return;
        }

        let action = self.determine_action();
        if action == RobotAction::None {
            exit(1);
        }
        match action {
            RobotAction::Deplacer(direction) => self.move_player(direction),
            RobotAction::Pickup => self.pickup().expect("Failed to pick up ingredient"),
            RobotAction::Deposit => self.deposit(now).expect("Failed to deposit ingredient"),
            RobotAction::None => (),
        }
    }

    pub fn determine_action(&self) -> RobotAction {
        let objectives = self.determine_objectives();
        let (x, y) = self.player.get_pos();

        for objective_level in objectives {
            // parmis un niveau d'objectif, choisir celui le plus proche
            let mut choosen_path: Vec<(usize, usize)> = Vec::new();
            let mut choosen_dist: usize = usize::MAX;
            for objective in objective_level {
                match self.pathfind_case((x, y), objective) {
                    Some(chemin) => {
                        if chemin.len() < choosen_dist {
                            choosen_dist = chemin.len();
                            choosen_path = chemin;
                        }
                    }
                    _ => continue,
                };
            }
            if choosen_dist == usize::MAX {
                continue;
            }

            let next_pos = match choosen_path.get(1) {
                Some(value) => *value,
                None => continue,
            };

            let direction = match next_pos {
                (x1, y1) if (x1, y1) == (x, y - 1) => Direction::North,
                (x1, y1) if (x1, y1) == (x, y + 1) => Direction::South,
                (x1, y1) if (x1, y1) == (x - 1, y) => Direction::West,
                (x1, y1) if (x1, y1) == (x + 1, y) => Direction::East,
                _ => continue,
            };

            if choosen_path.len() != 2 || self.player.get_facing() != direction {
                return RobotAction::Deplacer(direction);
            }

            if self.player.get_object_held() == PlayerHand::Nothing {
                return RobotAction::Pickup;
            } else {
                return RobotAction::Deposit;
            }
        }

        RobotAction::None
    }

    pub fn determine_objectives(&self) -> Vec<Vec<Case>> {
        let mut assiettes = vec![Assiette::new()];
        if let PlayerHand::Assiette(assiette) = self.player.get_object_held() {
            assiettes.push(assiette);
        }
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Case::Table(Some(PlayerHand::Assiette(assiette))) = self.map[y][x].clone() {
                    assiettes.push(assiette);
                }
            }
        }

        let mut assiette: Assiette = Assiette::new();
        // let mut assiette_hashset: HashSet<Ingredient> = HashSet::new();
        let mut diff = usize::MAX;
        let mut assiette_priv_recette: HashSet<Ingredient> = HashSet::new();
        let mut recette_priv_assiette: HashSet<Ingredient> = HashSet::new();
        let mut recette_hashset: HashSet<Ingredient> = HashSet::new();

        for a in assiettes.iter() {
            assiette = a.clone();
            let assiette_hashset = assiette.get_hashset();
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
                    recette_hashset = recette.get_ingredients().clone();
                }
            }
        }

        if diff == usize::MAX {
            return vec![];
        }

        if !assiette_priv_recette.is_empty() {
            // ingr dans assiette pas dans recette
            if let PlayerHand::Ingredient(held_ingredient) = self.player.get_object_held()
                && !recette_hashset.contains(&held_ingredient)
            {
                return vec![vec![Case::Table(None)]];
            }
            return vec![vec![Case::Table(Some(PlayerHand::Assiette(assiette)))]];
        } else if recette_priv_assiette.is_empty() {
            // assiette = recette
            return match self.player.get_object_held() {
                PlayerHand::Nothing => {
                    vec![vec![Case::Table(Some(PlayerHand::Assiette(assiette)))]]
                }
                PlayerHand::Assiette(held_assiette) if held_assiette.eq(&assiette) => {
                    vec![vec![Case::Depot(None)]]
                }
                _ => vec![vec![Case::Table(None)]],
            };
        }

        if let PlayerHand::Ingredient(held_ingredient) = self.player.get_object_held() {
            if recette_priv_assiette.contains(&held_ingredient) {
                // l'ingrédient dans la main est dans la recette mais pas dans l'assiette
                return vec![vec![
                    Case::Table(Some(PlayerHand::Assiette(assiette))),
                    Case::Assiette,
                ]];
            } else if recette_priv_assiette.iter().any(|ingr| {
                held_ingredient.type_ingredient.eq(&ingr.type_ingredient)
                    && held_ingredient.etat.eq(&IngredientEtat::Normal)
            }) {
                // ce qu'on a dans la main n'est pas sous la bonne forme
                return vec![vec![Case::Couper(None)]];
            } else if recette_priv_assiette.iter().any(|ingr| {
                held_ingredient.type_ingredient.eq(&ingr.type_ingredient) && ingr.cuisable.eq(&true)
            }) {
                // ce qu'on a dans la main n'a pas la bonne cuisson
                return vec![vec![Case::Cuire(None)]];
            } else {
                // ce qu'on a dans la main n'est pas dans la recette
                return vec![vec![Case::Table(None)]];
            }
        }

        if let PlayerHand::Assiette(_held_assiette) = self.player.get_object_held() {
            return vec![vec![Case::Table(None)]];
        }

        let mut recette_priv_assiette_vec = recette_priv_assiette.into_iter().collect::<Vec<_>>();

        // choisit l'ingredient qui apparait le plus dans les recettes d'apres (au cas où la recette actuelle se termine)
        recette_priv_assiette_vec.sort_by(|ingr1, ingr2| {
            let ingr1_count = self
                .recettes
                .iter()
                .filter(|recette| {
                    recette
                        .get_ingredients()
                        .iter()
                        .collect::<HashSet<_>>()
                        .contains(ingr1)
                })
                .count();
            let ingr2_count = self
                .recettes
                .iter()
                .filter(|recette| {
                    recette
                        .get_ingredients()
                        .iter()
                        .collect::<HashSet<_>>()
                        .contains(ingr2)
                })
                .count();
            // count par ordre décroissant et ingrédients par ordre croissant
            ingr2_count.cmp(&ingr1_count).then(ingr1.cmp(ingr2))
        });

        let next_ingredient = recette_priv_assiette_vec.first().unwrap();
        vec![
            vec![
                Case::Table(Some(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                }))),
                Case::Cuire(Some(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
            ],
            vec![Case::Table(Some(PlayerHand::Ingredient(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Coupe,
                cuisson: IngredientCuisson::Cru,
                cuisable: true,
            })))],
            vec![Case::Table(Some(PlayerHand::Ingredient(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Coupe,
                cuisson: IngredientCuisson::Cru,
                cuisable: false,
            })))],
            vec![
                Case::Table(Some(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                }))),
                Case::Cuire(Some(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
            ],
            vec![
                Case::Table(Some(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cru,
                    cuisable: false,
                }))),
                Case::Ingredient(next_ingredient.type_ingredient),
            ],
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

impl Default for Game {
    fn default() -> Self {
        Self::new("./src/map1.csv".to_string())
    }
}

// impl std::fmt::Display for Game {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (y, row) in self.map.iter().enumerate() {
//             let line = row
//                 .iter()
//                 .enumerate()
//                 .map(|(x, case)| match case {
//                     Case::Vide => {
//                         if (x, y) == self.player.get_pos() {
//                             "·".to_string()
//                         } else {
//                             " ".to_string()
//                         }
//                     }
//                     Case::Table(None) => "#".to_string(),
//                     Case::Table(Some(ingredient)) => ingredient.type_ingredient.char().to_string(),
//                     Case::Ingredient(ingredient_type) => ingredient_type.upper_char().to_string(),
//                     Case::COUPER => "C".to_string(),
//                     Case::DEPOT => "O".to_string(),
//                     Case::CUIRE => "F".to_string(),
// })
//                 .collect::<Vec<_>>()
//                 .join(" ");

//             writeln!(f, "{line}")?;
//         }
//         writeln!(f)?;

//         let line = self
//             .recettes
//             .iter()
//             .map(Recette::to_string)
//             .collect::<Vec<_>>()
//             .join("\n");
//         writeln!(f, "Recettes voulues : {}", line)?;

//         let line = self
//             .depot
//             .iter()
//             .map(Ingredient::to_string)
//             .collect::<Vec<_>>()
//             .join(", ");
//         writeln!(f, "Depot : {line}")?;

//         Ok(())
//     }
// }
