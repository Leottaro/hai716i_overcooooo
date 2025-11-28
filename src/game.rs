use crate::{
    COUPER_DURATION, CUIRE_DURATION, GAME_DURATION, RECETTE_COOLDOWN_RANGE, app_log,
    objets::{
        Assiette, Case, Direction, Ingredient, IngredientCuisson, IngredientEtat, IngredientType,
        Recette,
    },
    player::{Player, PlayerHand, PlayerIngredientStrategy, PlayerRecipeStrategy},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
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
    players: Vec<Player>,
    map: Vec<Vec<Case>>,
    recettes: Vec<Recette>,
    depot: Vec<(usize, Assiette)>,

    scores: Vec<i32>,
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
        for (i, result) in rdr.records().enumerate() {
            let record = result.expect("probleme lecture csv");

            for (j, r) in record.into_iter().enumerate() {
                let r = r.replace(" ", "").replace("\t", "");
                match r.as_str() {
                    "" => {
                        map[i][j] = Case::Vide;
                    }
                    "T" => {
                        map[i][j] = Case::Table(PlayerHand::Nothing);
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
                        map[i][j] = Case::Depot;
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
            }
        }
        map
    }

    pub fn new(file: String) -> Self {
        let map: Vec<Vec<Case>> = Game::lecture_map(file);

        let players = vec![
            Player::new(
                (1, 1),
                PlayerRecipeStrategy::Closest,
                PlayerIngredientStrategy::NbApparitionInRecipe,
            ),
            Player::new(
                (13, 8),
                PlayerRecipeStrategy::LatestExpiration,
                PlayerIngredientStrategy::Nearest,
            ),
        ];

        let scores = vec![0; players.len()];

        Self {
            players,
            map,
            recettes: vec![Recette::default_recipe()],
            depot: vec![],
            scores,
            next_recette: Instant::now() + rand::random_range(RECETTE_COOLDOWN_RANGE),
            end_instant: Instant::now() + GAME_DURATION,
            is_finished: false,
            cuire_events: HashMap::new(),
        }
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
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

    pub fn get_scores(&self) -> &Vec<i32> {
        &self.scores
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

    pub fn get_facing(&self, pos: (usize, usize), player: usize) -> ((usize, usize), Case) {
        let mut facing_pos: (usize, usize) = pos;
        let lenx: usize = self.map[0].len();
        let leny: usize = self.map.len();

        match self.players[player].get_facing() {
            Direction::North => facing_pos.1 = pos.1 - 1,
            Direction::West => facing_pos.0 = pos.0 - 1,
            Direction::South => facing_pos.1 = pos.1 + 1,
            Direction::East => facing_pos.0 = pos.0 + 1,
        }

        if facing_pos.0 >= lenx || facing_pos.1 >= leny {
            return (facing_pos, Case::Vide);
        }

        (facing_pos, self.map[facing_pos.1][facing_pos.0].clone())
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

    fn move_player(&mut self, direction: Direction, player: usize) {
        if self.players[player].is_blocked() {
            return;
        }
        self.players[player].set_facing(direction);
        let wanted_pos: (usize, usize) = self.get_facing(self.players[player].get_pos(), player).0;
        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide {
            self.players[player].set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
    }

    pub fn pickup(&mut self, player: usize) -> Result<(), PickupError> {
        if self.players[player].is_blocked() || self.is_finished {
            return Ok(());
        }

        let (facing_pos, facing_object) = self.get_facing(self.players[player].get_pos(), player);
        if self.players[player].get_object_held() != PlayerHand::Nothing {
            return Err(PickupError::HandsFull);
        }

        match facing_object.clone() {
            Case::Ingredient(object) => self.players[player]
                .set_object_held(PlayerHand::Ingredient(Ingredient::new(object))),
            Case::Table(PlayerHand::Nothing) => return Err(PickupError::TableEmpty),
            Case::Table(content) => {
                self.players[player].set_object_held(content.clone());
                self.map[facing_pos.1][facing_pos.0] = Case::Table(PlayerHand::Nothing);
            }
            Case::Assiette => self.players[player]
                .set_object_held(PlayerHand::Assiette((player, Assiette::new()))),
            Case::Cuire(Some(ingr)) => {
                self.players[player].set_object_held(PlayerHand::Ingredient(ingr));
                self.map[facing_pos.1][facing_pos.0] = Case::Cuire(None);
            }
            _ => return Err(PickupError::NoTarget((facing_pos, facing_object.clone()))),
        }

        Ok(())
    }

    pub fn deposit(&mut self, now: Instant, player: usize) -> Result<(), DepositError> {
        if self.players[player].is_blocked() || self.is_finished {
            return Ok(());
        }

        let (facing_pos, facing_object) = self.get_facing(self.players[player].get_pos(), player);
        match (
            self.players[player].get_object_held(),
            facing_object.clone(),
        ) {
            (hand, Case::Table(PlayerHand::Nothing)) => {
                self.map[facing_pos.1][facing_pos.0] = Case::Table(hand);
                self.players[player].set_object_held(PlayerHand::Nothing);
            }
            (
                PlayerHand::Ingredient(ingr),
                Case::Table(PlayerHand::Assiette((assiette_player, assiette))),
            ) => {
                if player == assiette_player {
                    let mut new_assiette = assiette.clone();
                    new_assiette.ingredients.push(ingr);
                    self.map[facing_pos.1][facing_pos.0] =
                        Case::Table(PlayerHand::Assiette((player, new_assiette)));
                    self.players[player].set_object_held(PlayerHand::Nothing);
                }
            }
            (_hand, Case::Table(_)) => {
                return Err(DepositError::TableFull);
            }
            (PlayerHand::Ingredient(ingredient), Case::Couper(None)) => {
                app_log!(
                    "Player {} commence à couper {:?}",
                    player,
                    ingredient.type_ingredient
                );
                self.players[player].set_object_held(PlayerHand::Nothing);
                self.players[player].block();
                self.map[facing_pos.1][facing_pos.0] = Case::Couper(Some(player));
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
                app_log!(
                    "Player {} commence à cuire {:?}",
                    player,
                    ingredient.type_ingredient
                );
                self.players[player].set_object_held(PlayerHand::Nothing);
                self.cuire_events.insert(
                    facing_pos,
                    Box::new(move |instant| {
                        if instant < now + CUIRE_DURATION {
                            Some(ingredient)
                        } else {
                            Some(ingredient.into_cuit())
                        }
                    }),
                );
                self.map[facing_pos.1][facing_pos.0] = Case::Cuire(Some(ingredient));
            }
            (PlayerHand::Assiette((_, assiette)), Case::Depot) => {
                app_log!(
                    "Player {} soumet une assiette avec {} ingrédients",
                    player,
                    assiette.ingredients.len()
                );
                self.depot.push((player, assiette.clone()));
                self.players[player].set_object_held(PlayerHand::Nothing);
            }
            (PlayerHand::Ingredient(ingredient), Case::Assiette) => {
                self.players[player].set_object_held(PlayerHand::Assiette((
                    player,
                    Assiette::create_with(ingredient),
                )));
            }
            (PlayerHand::Nothing, Case::Assiette) => {
                self.players[player]
                    .set_object_held(PlayerHand::Assiette((player, Assiette::new())));
            }
            (PlayerHand::Nothing, _) => return Ok(()),
            _ => return Err(DepositError::NoTarget((facing_pos, facing_object))),
        }

        Ok(())
    }

    pub fn tick(&mut self, now: Instant) {
        if self.is_finished {
            return;
        }

        // Update the too lates recettes
        self.recettes = self
            .recettes
            .iter()
            .filter(|recette| !recette.is_too_late(now))
            .cloned()
            .collect();

        if self.next_recette <= now || self.recettes.len() < 2 {
            self.add_random_recette(now);
            if self.next_recette <= now {
                self.next_recette = now + rand::random_range(RECETTE_COOLDOWN_RANGE);
            }
        }

        // Update the depot vector
        for (player, assiette) in self.depot.drain(..) {
            let recette_correspondante = self
                .recettes
                .iter()
                .position(|recette| assiette.get_hashset().eq(recette.get_ingredients()));
            if let Some(i) = recette_correspondante {
                let bonus = assiette.ingredients.len() * assiette.ingredients.len()
                    - 2 * assiette.ingredients.len()
                    + 4;
                self.scores[player] += bonus as i32;
                app_log!(
                    "Recette validée par joueur {} ! +{} points (total: {})",
                    player,
                    bonus,
                    self.scores[player]
                );
                self.recettes.remove(i);
            }
        }

        // Update the Case
        for (y, line) in self.map.iter_mut().enumerate() {
            for (x, case) in line.iter_mut().enumerate() {
                match case {
                    Case::Couper(Some(_)) => {
                        if let Some(ingredient) =
                            self.cuire_events.get(&(x, y)).and_then(|func| func(now))
                        {
                            self.players.iter_mut().for_each(|player| {
                                player.set_object_held(PlayerHand::Ingredient(ingredient))
                            });
                            self.players.iter_mut().for_each(|player| player.unblock());
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
                    _ => (),
                }
            }
        }

        if self.end_instant <= now {
            self.is_finished = true;
        }
    }

    pub fn robot(&mut self, now: Instant, player: usize) {
        if self.is_finished {
            return;
        }

        let action = self.determine_action(player);
        match action {
            RobotAction::Deplacer(direction) => self.move_player(direction, player),
            RobotAction::Pickup => {
                if let Err(e) = self.pickup(player) {
                    app_log!("Robot {}: Pickup failed - {:?}", player, e);
                }
            }
            RobotAction::Deposit => {
                if let Err(e) = self.deposit(now, player) {
                    app_log!("Robot {}: Deposit failed - {:?}", player, e);
                }
            }
            RobotAction::None => (),
        }
    }

    pub fn determine_action(&self, player: usize) -> RobotAction {
        let objectives = self.determine_objectives(player);
        let (x, y) = self.players[player].get_pos();

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

            if choosen_path.len() != 2 || self.players[player].get_facing() != direction {
                return RobotAction::Deplacer(direction);
            }

            if self.players[player].get_object_held() == PlayerHand::Nothing {
                return RobotAction::Pickup;
            } else {
                return RobotAction::Deposit;
            }
        }

        RobotAction::None
    }

    pub fn determine_objectives(&self, player: usize) -> Vec<Vec<Case>> {
        let (assiette, recette, diff) = match self.players[player].get_recipes_strategy() {
            PlayerRecipeStrategy::Closest => self.determine_closest_assiette_recette(player),
            PlayerRecipeStrategy::LatestExpiration => {
                self.determine_latest_assiette_recette(player)
            }
        };

        if diff == usize::MAX {
            return vec![];
        }

        if diff == 0 {
            // assiette = recette
            match self.players[player].get_object_held() {
                PlayerHand::Assiette((_, held_assiette)) if held_assiette.eq(&assiette) => {
                    // j'ai la bonne assiette dans la main, je dois aller la déposer
                    return vec![vec![Case::Depot]];
                }
                PlayerHand::Nothing => {
                    // je dois aller chercher la bonne assiette
                    return vec![vec![Case::Table(PlayerHand::Assiette((player, assiette)))]];
                }
                _ => {
                    // j'ai autre chose dans la main, je dois le poser pour prendre la bonne assiette
                    return vec![vec![Case::Table(PlayerHand::Nothing)]];
                }
            };
        }

        if assiette
            .get_hashset()
            .difference(recette.get_ingredients())
            .count()
            > 0
        {
            // il y a des ingrédients en trop dans l'assiette par rapport à la recette

            if let PlayerHand::Ingredient(held_ingredient) = self.players[player].get_object_held()
                && !recette.get_ingredients().contains(&held_ingredient)
            {
                // j'ai un ingrédient dans la main qui n'est pas dans la recette, je dois le poser
                return vec![vec![Case::Table(PlayerHand::Nothing)]];
            }

            // C'est plus rapide d'aller chercher cette assiette et de vider les ingrédients en trop que d'en construire une nouvelle
            return vec![vec![Case::Table(PlayerHand::Assiette((player, assiette)))]];
        }

        // On a rien return donc il manque des ingrédients dans l'assiette par rapport à la recette

        let recette_priv_assiette = recette
            .get_ingredients()
            .difference(&assiette.get_hashset())
            .cloned()
            .collect::<HashSet<_>>();

        match self.players[player].get_object_held() {
            PlayerHand::Ingredient(held_ingredient) => {
                if recette_priv_assiette.contains(&held_ingredient) {
                    // l'ingrédient dans la main est dans la recette mais pas dans l'assiette
                    return vec![vec![
                        Case::Table(PlayerHand::Assiette((player, assiette))),
                        Case::Assiette,
                    ]];
                } else if recette_priv_assiette.iter().any(|ingr| {
                    held_ingredient.type_ingredient.eq(&ingr.type_ingredient)
                        && held_ingredient.etat.eq(&IngredientEtat::Normal)
                }) {
                    // ce qu'on a dans la main n'est pas sous la bonne forme
                    return vec![vec![Case::Couper(None)]];
                } else if recette_priv_assiette.iter().any(|ingr| {
                    held_ingredient.type_ingredient.eq(&ingr.type_ingredient)
                        && ingr.cuisable.eq(&true)
                }) {
                    // ce qu'on a dans la main n'a pas la bonne cuisson
                    return vec![vec![Case::Cuire(None)]]; // TODO: il y a moyen qu'il se stuck ici
                }

                // ce qu'on a dans la main n'est pas dans la recette
                return vec![vec![Case::Table(PlayerHand::Nothing)]];
            }
            PlayerHand::Assiette(_assiette) => return vec![vec![Case::Table(PlayerHand::Nothing)]],
            PlayerHand::Nothing => match self.players[player].get_ingredients_strategy() {
                PlayerIngredientStrategy::NbApparitionInRecipe => {
                    Game::list_objectives_distance(recette_priv_assiette)
                }
                PlayerIngredientStrategy::Nearest => {
                    self.list_objectives_count(recette_priv_assiette)
                }
            },
        }
    }

    // ======================================================================================================
    // ============================================= STRATEGIES =============================================
    // ======================================================================================================

    fn determine_latest_assiette_recette(&self, player: usize) -> (Assiette, Recette, usize) {
        let mut assiettes = vec![];
        if let PlayerHand::Assiette((_, assiette)) = self.players[player].get_object_held() {
            assiettes.push(assiette);
        }
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Case::Table(PlayerHand::Assiette((assiette_player, assiette))) =
                    self.map[y][x].clone()
                {
                    if player == assiette_player {
                        assiettes.push(assiette);
                    }
                }
            }
        }
        assiettes.sort_by_key(|a| a.ingredients.len());

        let mut recettes = self.recettes.clone();
        recettes.sort_by_key(|r| r.expiration);

        for assiette in assiettes.into_iter().rev() {
            for recette in self.recettes.iter().rev() {
                let assiette_hashset = assiette.get_hashset();
                if assiette_hashset.is_subset(&recette.ingredients) {
                    let recette_priv_assiette = recette
                        .ingredients
                        .difference(&assiette_hashset)
                        .collect::<HashSet<_>>();
                    return (assiette, recette.clone(), recette_priv_assiette.len());
                }
            }
        }

        let recette = self.recettes.last().unwrap();
        (Assiette::new(), recette.clone(), recette.ingredients.len())
    }

    fn determine_closest_assiette_recette(&self, player: usize) -> (Assiette, Recette, usize) {
        // TODO: TOFIX (il pause plein d'assiettes partout)
        let mut assiettes = vec![Assiette::new()];
        if let PlayerHand::Assiette((_, assiette)) = self.players[player].get_object_held() {
            assiettes.push(assiette);
        }
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Case::Table(PlayerHand::Assiette((assiette_player, assiette))) =
                    self.map[y][x].clone()
                {
                    if player == assiette_player {
                        assiettes.push(assiette);
                    }
                }
            }
        }

        let mut assiette: &Assiette = &Assiette::new();
        let mut recette: &Recette = &Recette::new(Instant::now());
        let mut diff = usize::MAX;

        for a in assiettes.iter() {
            assiette = a;
            let assiette_hashset = assiette.get_hashset();
            for current_recette in self.recettes.iter() {
                let recette_priv_assiette = current_recette
                    .get_ingredients()
                    .difference(&assiette_hashset)
                    .cloned()
                    .collect::<HashSet<_>>();
                let assiette_priv_recette = assiette_hashset
                    .difference(current_recette.get_ingredients())
                    .cloned()
                    .collect::<HashSet<_>>();
                let current_diff = assiette_priv_recette.len() + recette_priv_assiette.len();
                if current_diff < diff {
                    diff = current_diff;
                    recette = current_recette;
                }
            }
        }

        (assiette.clone(), recette.clone(), diff)
    }

    fn list_objectives_distance(ingredients: HashSet<Ingredient>) -> Vec<Vec<Case>> {
        // ORDRE logique de priorité:
        // 0: coupé cuit (cuisable)
        // 1: coupé cru (pas cuisable)
        // 2: coupé cru (cuisable)
        // 3: normal cru (cuisable)
        // 4: normal cru (pas cuisable)

        // TODO: TOFIX: il est con
        let mut objectives = vec![vec![]; 5];

        for next_ingredient in ingredients.into_iter() {
            if next_ingredient.cuisable {
                objectives[0].extend(vec![
                    Case::Table(PlayerHand::Ingredient(Ingredient {
                        type_ingredient: next_ingredient.type_ingredient,
                        etat: IngredientEtat::Coupe,
                        cuisson: IngredientCuisson::Cuit,
                        cuisable: true,
                    })),
                    Case::Cuire(Some(Ingredient {
                        type_ingredient: next_ingredient.type_ingredient,
                        etat: IngredientEtat::Coupe,
                        cuisson: IngredientCuisson::Cuit,
                        cuisable: true,
                    })),
                ]);
                objectives[2].push(Case::Table(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cru,
                    cuisable: true,
                })));
                objectives[3].extend(vec![
                    Case::Table(PlayerHand::Ingredient(Ingredient {
                        type_ingredient: next_ingredient.type_ingredient,
                        etat: IngredientEtat::Normal,
                        cuisson: IngredientCuisson::Cuit,
                        cuisable: true,
                    })),
                    Case::Cuire(Some(Ingredient {
                        type_ingredient: next_ingredient.type_ingredient,
                        etat: IngredientEtat::Normal,
                        cuisson: IngredientCuisson::Cuit,
                        cuisable: true,
                    })),
                ]);
            } else {
                objectives[1].push(Case::Table(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cru,
                    cuisable: false,
                })));
                objectives[4].extend(vec![
                    Case::Table(PlayerHand::Ingredient(Ingredient {
                        type_ingredient: next_ingredient.type_ingredient,
                        etat: IngredientEtat::Normal,
                        cuisson: IngredientCuisson::Cru,
                        cuisable: false,
                    })),
                    Case::Ingredient(next_ingredient.type_ingredient),
                ]);
            }
        }

        objectives
    }

    fn list_objectives_count(&self, ingredients: HashSet<Ingredient>) -> Vec<Vec<Case>> {
        let mut ingredients_vec = ingredients.into_iter().collect::<Vec<_>>();

        // choisit l'ingredient qui apparait le plus dans les recettes d'apres (au cas où la recette actuelle se termine)
        ingredients_vec.sort_by(|ingr1, ingr2| {
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
            // cuisable d'abord puis count par ordre décroissant puis ingrédients par ordre croissant
            ingr2
                .cuisable
                .cmp(&ingr1.cuisable)
                .then(ingr2_count.cmp(&ingr1_count).then(ingr1.cmp(ingr2)))
        });

        let mut ingredients_pool = ingredients_vec.into_iter();
        let mut next_ingredient = ingredients_pool.next().unwrap();

        // Si le next ingrédient est entrain d'être cuit alors on le skip (on l'aura plus tard)
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                let case = &self.map[y][x];
                if let Case::Cuire(Some(cuire_ingr)) = case {
                    if cuire_ingr.type_ingredient == next_ingredient.type_ingredient
                        && cuire_ingr.cuisson != IngredientCuisson::Cuit
                    {
                        next_ingredient = match ingredients_pool.next() {
                            Some(ingr) => ingr,
                            None => return vec![],
                        }
                    }
                }
            }
        }

        vec![
            vec![
                Case::Table(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
                Case::Cuire(Some(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Coupe,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
            ],
            vec![Case::Table(PlayerHand::Ingredient(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Coupe,
                cuisson: IngredientCuisson::Cru,
                cuisable: true,
            }))],
            vec![Case::Table(PlayerHand::Ingredient(Ingredient {
                type_ingredient: next_ingredient.type_ingredient,
                etat: IngredientEtat::Coupe,
                cuisson: IngredientCuisson::Cru,
                cuisable: false,
            }))],
            vec![
                Case::Table(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
                Case::Cuire(Some(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cuit,
                    cuisable: true,
                })),
            ],
            vec![
                Case::Table(PlayerHand::Ingredient(Ingredient {
                    type_ingredient: next_ingredient.type_ingredient,
                    etat: IngredientEtat::Normal,
                    cuisson: IngredientCuisson::Cru,
                    cuisable: false,
                })),
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
//                     Case::Table(PlayerHand::Nothing) => "#".to_string(),
//                     Case::Table((ingredient)) => ingredient.type_ingredient.char().to_string(),
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
