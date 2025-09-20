use std::{collections::HashSet, usize};
use crate::{
    objets::{Case, Direction, Ingredient, IngredientType, Recette, RobotAction},
    player::Player, RECETTE_RANGE
};

#[derive(Debug, PartialEq)]
pub struct Game {
    player: Player,
    assiette: Vec<Ingredient>,
    map: Vec<Vec<Case>>,
    recettes: Vec<Recette>,
    
    score: i32,
    next_recette: usize,
    t: usize,
    max_t: usize,
}

impl Game {
    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_assiette(&self) -> &Vec<Ingredient> {
        &self.assiette
    }
        
    pub fn get_map(&self) -> &Vec<Vec<Case>> {
        &self.map
    }


    pub fn get_recettes(&self) -> &Vec<Recette> {
        &self.recettes
    }

    pub fn get_map_heigth(&self) -> usize {
        self.map.len()
    }

    pub fn get_map_width(&self) -> usize {
        self.map[0].len()
    }

    pub fn new(max_t: usize) -> Self {
        let map: Vec<Vec<Case>> = vec![
            vec![Case::Table(None), Case::Table(None), Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None), Case::Table(None),Case::Table(None),Case::ASSIETTE, Case::Table(None)],
            vec![Case::Ingredient(IngredientType::Pain), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Ingredient(IngredientType::Tomate), Case::Table(None),Case::Ingredient(IngredientType::Salade),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None) ,Case::Table(None),Case::Ingredient(IngredientType::Oignon),Case::Table(None),Case::Table(None),]        ];
        
        Self {
            player: Player::new((1, 1)),
            map,
            recettes: vec![Recette::new(), Recette::new()],
            assiette: Vec::new(),
            score: 0,
            t: 0,
            max_t,
            next_recette: rand::random_range(RECETTE_RANGE),
        }
    }

    fn get_facing(&self, pos: (usize, usize)) -> ((usize, usize), Case) {
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
        return (facing_pos, self.map[facing_pos.1][facing_pos.0]);
    }
    
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x-1, y));
        }
        if y > 0 {
            neighbours.push((x, y-1));
        }
        if x < self.map[0].len() - 1 {
            neighbours.push((x+1, y));
        }
        if y < self.map.len() - 1 {
            neighbours.push((x, y+1));
        }
        neighbours
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.set_facing(direction);
        let wanted_pos: (usize, usize) = self.get_facing(self.player.get_pos()).0;
        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide  {
            self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
        
    }
    
    pub fn pickup(&mut self) {
        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        
        let object_held = self.player.get_object_held();
        
        if object_held != None{
            return;
        }
        
        match facing_object {
            Case::ASSIETTE => {
                        self.player.set_object_held(self.assiette.pop());
                    }
            Case::Ingredient(object) => {
                        self.player.set_object_held(Some(Ingredient::new(object)));
                    }
            Case::Table(None) => {}
            Case::Table(ingredient) => {
                        self.player.set_object_held(ingredient);
                        self.map[facing_pos.1][facing_pos.0] = Case::Table(None);
                    }
            _ => {}
        }
    }
    
    pub fn deposit(&mut self) {        
        let (facing_pos, facing_object) = self.get_facing(self.player.get_pos());
        let mut object_held = match self.player.take_object_held() {
            None => return,
            Some(obj) => obj,
        };

        match facing_object {
            Case::ASSIETTE => {
                        self.assiette.push(object_held);
                        let recette_correspondante = self.recettes.iter().position(|recette| self.assiette.eq(recette.get_ingredients()));
                        if let Some(i) = recette_correspondante {
                            self.score += self.assiette.len() as i32;
                            self.assiette = vec![];
                            self.recettes.remove(i);
                        }
                    }
            Case::Table(None) => {
                        self.map[facing_pos.1][facing_pos.0] = Case::Table(Some(object_held));
                    }
            Case::Table(_) => {}
            Case::COUPER => {
                object_held.couper();
                self.player.set_object_held(Some(object_held));
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if self.t >= self.max_t {
            return;
        }
        self.t += 1;
        
        let mut removed_recettes = Vec::new();
        self.recettes.retain_mut(|recette| {
            recette.pass_time();
            if recette.is_too_late() {
                removed_recettes.push(recette.clone());
                false
            } else {
                true
            }
        });
        self.score -= removed_recettes.len() as i32;

        self.next_recette -= 1;
        if self.next_recette == 0 {
            self.recettes.push(Recette::new());
            self.next_recette = rand::random_range(RECETTE_RANGE);
        }
    }

    pub fn robot(&mut self) {
        let action = self.determine_action();
        match action {
            RobotAction::Deplacer(direction) => {
                self.move_player(direction)
            },
            RobotAction::Pickup => {
                self.pickup()
            },
            RobotAction::Deposit => {
                self.deposit()
            },
            RobotAction::None => (),
        }
    }

    fn determine_action(&self) -> RobotAction {
        let next_recette = match self.recettes.last() {
            None => return RobotAction::None,
            Some(recette) => recette,
        };

        let mut ingredients_restant = next_recette.get_ingredients().clone();
        for ingredient in self.assiette.iter() {
            if let Some(i) = ingredients_restant.iter().position(|ingr| ingr.eq(ingredient)) {
                ingredients_restant.remove(i);
            }
        }

        let next_ingredient = match ingredients_restant.iter().next() {
            None => return RobotAction::None,
            Some(ingr) => ingr.clone(),
        };
        
        let (x, y) = self.player.get_pos();
        let objective = match self.player.get_object_held() {
            None => Case::Ingredient(next_ingredient.type_ingredient),
            Some(ingr) => {
                if ingr.etat == next_ingredient.etat {
                    Case::ASSIETTE
                } else {
                    Case::COUPER
                }
            },
        };

        // let recipes_str = self.recettes.iter().map(Recette::to_string).collect::<Vec<_>>().join("\n");
        // let assiette_str = self.assiette.iter().map(Ingredient::to_string).collect::<Vec<_>>().join(", ");
        // let player_str = format!("{:?}", self.player);
        // let objective_str = format!("{:?}", objective);
        // println!("\nASSIETTE: {assiette_str}\nRECIPES: {recipes_str}\nPLAYER: {player_str}\nOBJECTIVE: {objective_str}");

        let chemin = match self.pathfind_case((x, y), objective) {
            None => return RobotAction::None,
            Some(chemin) => chemin,
        };
        
        let next_pos = match chemin.get(1) {
            Some(value) => value.clone(),
            None => return RobotAction::None,
        };

        let direction = match next_pos {
            (x1, y1) if (x1, y1) == (x, y - 1)  => Direction::North,
            (x1, y1) if (x1, y1) == (x, y + 1)  => Direction::South,
            (x1, y1) if (x1, y1) == (x - 1, y)  => Direction::West,
            (x1, y1) if (x1, y1) == (x + 1, y)  => Direction::East,
            _ => return RobotAction::None,
        };
        
        if chemin.len() != 2 || self.player.get_facing() != direction {
            // println!("ACTION: {:?}", RobotAction::Deplacer(direction));
            return RobotAction::Deplacer(direction);
        }

        if self.player.get_object_held().is_none() {
            // println!("ACTION: {:?}", RobotAction::Pickup);
            RobotAction::Pickup
        } else {
            // println!("ACTION: {:?}", RobotAction::Deposit);
            RobotAction::Deposit
        }
    }

    fn pathfind_case(&self, start: (usize, usize), case: Case) -> Option<Vec<(usize, usize)>> {
        let mut weights: Vec<Vec<usize>> = vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        let mut explored_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut next_positions: Vec<(usize, usize)> = vec![start];
        weights[start.1][start.0] = 0;
        // println!("WEIGHTS:\n{}", weights.iter().map(|line| line.iter().map(|weight| if *weight == usize::MAX {"XX".to_string()} else {format!("{weight:2}")}).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n"));
        
        let mut found_pos: Option<(usize, usize)> = None;
        while let Some((x, y)) = next_positions.pop() {
            if !explored_positions.insert((x,y)) {
                continue;
            }

            let mut min_neighbour = usize::MAX;
            for (x1, y1) in self.get_neighbours(x, y) {
                if weights[y1][x1] == usize::MAX {
                    if self.map[y1][x1] == Case::Vide {
                        next_positions.insert(0, (x1,y1));
                    } else if self.map[y1][x1] == case {
                        found_pos = Some((x1, y1));
                    } 
                } else if weights[y1][x1] < min_neighbour  {
                    min_neighbour = weights[y1][x1];
                }
            }

            if min_neighbour != usize::MAX {
                weights[y][x] = min_neighbour + 1;
                // println!("WEIGHTS: (checked {x},{y})\n{}", weights.iter().map(|line| line.iter().map(|weight| if *weight == usize::MAX {"XX".to_string()} else {format!("{weight:2}")}).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n"));
            } 

            if found_pos.is_some() {
                break;
            }
        }

        if found_pos.is_none() {
            return None;
        }
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

        // let path_str = path.iter().map(|(x,y)| format!("({x},{y})")).collect::<Vec<_>>().join(", ");
        // let weights_str = weights.into_iter().map(|line| line.into_iter().map(|weight| if weight == usize::MAX {"XX".to_string()} else {format!("{weight:2}")}).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n");
        // println!("PATH: {path_str}\nWEIGHTS:\n{weights_str}");

        Some(path)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            let line = row
                .iter().enumerate()
                .map(|(x, case)| match case {
                    Case::Vide => if (x, y) == self.player.get_pos() {"·".to_string()} else {" ".to_string()},
                    Case::Table(None) => "#".to_string(),
                    Case::Table(Some(ingredient)) => ingredient.type_ingredient.char().to_string(),
                    Case::Ingredient(ingredient_type) => ingredient_type.upper_char().to_string(),
                    Case::COUPER => "C".to_string(),
                    Case::ASSIETTE => "O".to_string(),
                }).collect::<Vec<_>>()
                .join(" ");

            writeln!(f, "{line}")?;
        }
        writeln!(f)?;

        let line = self
            .recettes
            .iter()
            .map(Recette::to_string)
            .collect::<Vec<_>>().join("\n");
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
