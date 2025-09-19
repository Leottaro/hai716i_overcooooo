use std::collections::HashSet;

use crate::{
    objets::{Case, Direction, Ingredient, IngredientEtat, IngredientType, Recette, RobotAction},
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

    recent_path: Vec<(usize, usize)>
}

impl Game {
    pub fn get_player(&self) -> &Player {
        &self.player
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
            recent_path: Vec::new(),
        }
    }

    fn move_coords(&self, direction: Direction, coords: (usize, usize)) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if coords.1 > 0 {
                    return Some((coords.0, coords.1 - 1));
                }
            },
            Direction::West => {
                if coords.0 > 0 {
                    return Some((coords.0 - 1, coords.1));
                }
            },
            Direction::South => {
                if coords.1 < self.map.len()-1 {
                    return Some((coords.0, coords.1 + 1));
                }
            },
            Direction::East => {
                if coords.0 < self.map.len()-1 {
                    return Some((coords.0 + 1, coords.1));
                }
            },
        }
        return None;
    }
    
    pub fn move_player(&mut self, direction: Direction) {
        self.player.set_facing(direction);
        let wanted_pos: (usize, usize) = self.get_facing().0;
        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide  {
            self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
        
    }

    pub fn get_facing(&self) -> ((usize, usize), Case) {
        let pos: (usize, usize) = self.player.get_pos();
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
    
    pub fn pickup(&mut self) {
        let (facing_pos, facing_object) = self.get_facing();
        
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
        let (facing_pos, facing_object) = self.get_facing();
        let mut object_held = match self.player.get_object_held() {
            None => return,
            Some(obj) => obj,
        };

        match facing_object {
            Case::ASSIETTE => {
                        self.assiette.push(object_held);
                        for i in 0..self.recettes.len() {
                            if self.assiette == *self.recettes[i].get_ingredients() {
                                self.score += self.assiette.len() as i32;
                                self.assiette = vec![];
                                self.recettes.remove(i);
                            }
                        }
                    }
            Case::Table(None) => {
                        self.map[facing_pos.1][facing_pos.0] = Case::Table(self.player.get_object_held());
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
        if self.recent_path.len() == 1 {
            let (x, y) = self.player.get_pos();
            let next_pos = self.recent_path.remove(0);
            let direction = match next_pos {
                (x1, y1) if (x1, y1) == (x, y - 1)  => Direction::North,
                (x1, y1) if (x1, y1) == (x, y + 1)  => Direction::South,
                (x1, y1) if (x1, y1) == (x - 1, y)  => Direction::West,
                (x1, y1) if (x1, y1) == (x + 1, y)  => Direction::East,
                _ => panic!("aaaaaa"),
            };
            self.move_player(direction)
        }

        let action = self.determine_action();
        match action {
            RobotAction::Deplacer(chemin, direction) => {
                self.recent_path = chemin;
                self.move_player(direction)
            },
            RobotAction::Pickup => {
                self.pickup()
            },
            RobotAction::Deposit => 
            {
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

        let mut ingredients_restant = next_recette.get_ingredients().clone().into_iter().collect::<HashSet<_>>();
        for ingredient in &self.assiette {
            ingredients_restant.remove(ingredient);
        }
        
        let (x, y) = self.player.get_pos();
        let chemin = match self.player.get_object_held() {
            None => {
                let next_ingredient = match ingredients_restant.iter().next() {
                    None => return RobotAction::None,
                    Some(ingr) => ingr.clone(),
                };

                let chemin = match self.pathfind_case(vec![(x, y)], Case::Ingredient(next_ingredient.type_ingredient)) {
                    None => return RobotAction::None,
                    Some(chemin) => chemin,
                };
                chemin
            },
            Some(ingr) => {
                if ingr.etat == IngredientEtat::Coupe {
                    let chemin = match self.pathfind_case(vec![(x, y)], Case::ASSIETTE) {
                        None => return RobotAction::None,
                        Some(chemin) => chemin,
                    };
                    chemin
                } else {
                    let chemin = match self.pathfind_case(vec![(x, y)], Case::COUPER) {
                        None => return RobotAction::None,
                        Some(chemin) => chemin,
                    };
                    chemin
                }
            },
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
        
        if chemin.len() != 2 || self.player.get_facing() == direction {
            return RobotAction::Deplacer(chemin, direction);
        }

        if self.player.get_object_held().is_none() {
            RobotAction::Pickup
        } else {
            RobotAction::Deposit
        }
    }

    fn pathfind_case(&self, current_path: Vec<(usize, usize)>, case: Case) -> Option<Vec<(usize, usize)>> {
        let (x, y) = current_path.last().unwrap().clone();
        
        if self.map[y][x] == case {
            return Some(current_path);
        }

        if self.map[y][x] != Case::Vide {
            return None;
        }

        for direction in vec![Direction::North, Direction::West, Direction::South, Direction::East] {
            let next_case = self.move_coords(direction, (x, y));
            match next_case {
                None => (),
                Some(value) => {
                    if current_path.contains(&value) {
                        continue;
                    }

                    let mut next_path = current_path.clone();
                    next_path.push(value);
                    
                    match self.pathfind_case(next_path, case) {
                        None => (),
                        Some(chemin) => return Some(chemin),
                    }
                },
            }
        }

        return None;
    }
}

// impl std::fmt::Display for Game {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (y, row) in self.map.iter().enumerate() {
//             let line = row
//                 .iter().enumerate()
//                 .map(|(x, case)| match case {
//                     Case::Vide => if (x, y) == self.player.get_pos() {'·'} else {' '},
//                     Case::Table(None) => '#',
//                     Case::Table(Some(ingredient)) => ingredient.char(),
//                     Case::Ingredient(ingredient) => ingredient.upper_char(),
//                     Case::COUPER => 'C',
//                     Case::ASSIETTE => 'O',
//                 })
//                 .fold(String::new(), |c1, c2| format!("{c1} {c2}"));

//             writeln!(f, "{line}")?;
//         }
//         writeln!(f)?;

//         let line = self
//             .recettes
//             .iter()
//             .map(Recette::to_string)
//             .fold(String::new(), |r1, r2| format!("{r1} | {r2}"));
//         writeln!(f, "Recettes voulues : {}", line)?;

//         let line = self
//             .assiette
//             .iter()
//             .map(Ingredient::to_string)
//             .fold(String::new(), |i1, i2| format!("{i1},{i2}"));
//         writeln!(f, "Assiette : {line}")?;

//         Ok(())
//     }
// }
