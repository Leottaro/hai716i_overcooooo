use std::collections::HashSet;

use crate::{
    objets::{ActionResult, Case, Direction, Ingredient, IngredientType, Recette}, player::Player, RECETTE_RANGE
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
        let pain = Ingredient::new(IngredientType::Pain);
        let salade = Ingredient::new(IngredientType::Salade);
        let tomate = Ingredient::new(IngredientType::Tomate);
        let oignon = Ingredient::new(IngredientType::Oignon);

        let map: Vec<Vec<Case>> = vec![
            vec![Case::Table(None), Case::Table(None), Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None), Case::Table(None),Case::Table(None),Case::ASSIETTE, Case::Table(None)],
            vec![Case::Ingredient(pain), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Ingredient(tomate), Case::Table(None),Case::Ingredient(salade),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None) ,Case::Table(None),Case::Ingredient(oignon),Case::Table(None),Case::Table(None),]        ];
        
        Self {
            player: Player::new((1, 1)),
            map,
            recettes: Vec::new(),
            assiette: Vec::new(),
            score: 0,
            t: 0,
            max_t,
            next_recette: rand::random_range(RECETTE_RANGE),
        }
    }


    
    pub fn move_player(&mut self, direction: Direction) -> ActionResult {
        let check = self.check_move(direction);
        if check == ActionResult::Success {
            self.player.set_facing(direction);
            let (wanted_pos, _) = self.get_target_position(direction);
            self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
        check
    }

    pub fn check_move(&self, direction: Direction) -> ActionResult {
        let (wanted_pos, _) = self.get_target_position(direction);
        
        if !self.is_position_valid(wanted_pos) {
            return ActionResult::InvalidPosition;
        }
        
        if self.map[wanted_pos.1][wanted_pos.0] != Case::Vide {
            return ActionResult::Blocked;
        }
        
        ActionResult::Success
    }

    fn get_target_position(&self, direction: Direction) -> ((usize, usize), Case) {
        let pos = self.player.get_pos();
        let mut target_pos = pos;
        
        match direction {
            Direction::North if pos.1 > 0 => target_pos.1 -= 1,
            Direction::South if pos.1 < self.map.len() - 1 => target_pos.1 += 1,
            Direction::West if pos.0 > 0 => target_pos.0 -= 1,
            Direction::East if pos.0 < self.map[0].len() - 1 => target_pos.0 += 1,
            _ => return (pos, Case::Vide), // Position invalide
        }
        
        (target_pos, self.map[target_pos.1][target_pos.0])
    }
    
    fn is_position_valid(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.map[0].len() && pos.1 < self.map.len()
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
        
        (facing_pos, self.map[facing_pos.1][facing_pos.0])
    }

    pub fn pickup(&mut self) -> ActionResult {
        let check = self.check_pickup();
        if check != ActionResult::Success {
            return check;
        }
        
        let (facing_pos, facing_object) = self.get_facing();
        
        match facing_object {
            Case::ASSIETTE => {
                if let Some(ingredient) = self.assiette.pop() {
                    self.player.set_object_held(Some(ingredient));
                    ActionResult::Success
                } else {
                    ActionResult::NoTarget
                }
            }
            Case::Ingredient(object) => {
                self.player.set_object_held(Some(object));
                self.map[facing_pos.1][facing_pos.0] = Case::Vide;
                ActionResult::Success
            }
            Case::Table(Some(ingredient)) => {
                self.player.set_object_held(Some(ingredient));
                self.map[facing_pos.1][facing_pos.0] = Case::Table(None);
                ActionResult::Success
            }
            _ => ActionResult::NoTarget
        }
    }

    pub fn check_pickup(&self) -> ActionResult {
        if self.player.get_object_held().is_some() {
            return ActionResult::HandsFull;
        }
        
        let (_, facing_object) = self.get_facing();
        match facing_object {
            Case::Ingredient(_) | Case::Table(Some(_)) | Case::ASSIETTE => ActionResult::Success,
            _ => ActionResult::NoTarget,
        }
    }

    pub fn deposit(&mut self) -> ActionResult {
        let check = self.check_deposit();
        if check != ActionResult::Success {
            return check;
        }
        
        let (facing_pos, facing_object) = self.get_facing();
        let object_held = self.player.get_object_held().unwrap(); // Safe car on a vérifié dans check_deposit
        
        match facing_object {
            Case::ASSIETTE => {
                self.assiette.push(object_held);
                self.player.set_object_held(None);
                
                // Vérifier si on a complété une recette
                for i in 0..self.recettes.len() {
                    if self.assiette == *self.recettes[i].get_ingredients() {
                        self.score += self.assiette.len() as i32;
                        self.assiette = vec![];
                        self.recettes.remove(i);
                        break;
                    }
                }
                ActionResult::Success
            }
            Case::Table(None) => {
                self.map[facing_pos.1][facing_pos.0] = Case::Table(Some(object_held));
                self.player.set_object_held(None);
                ActionResult::Success
            }
            Case::COUPER => {
                let mut ingredient = object_held;
                ingredient.couper();
                self.player.set_object_held(Some(ingredient));
                ActionResult::Success
            }
            _ => ActionResult::NoTarget
        }
    }

    pub fn check_deposit(&self) -> ActionResult {
        if self.player.get_object_held().is_none() {
            return ActionResult::HandsEmpty;
        }
        
        let (_, facing_object) = self.get_facing();
        match facing_object {
            Case::Table(None) | Case::ASSIETTE | Case::COUPER => ActionResult::Success,
            Case::Table(Some(_)) => ActionResult::TableOccupied,
            _ => ActionResult::NoTarget,
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

        if self.t == self.next_recette {
            self.recettes.push(Recette::new());
            self.next_recette = rand::random_range(RECETTE_RANGE);
        }
    }

    pub fn robot(&mut self) {
        let next_recette = match self.recettes.last() {
            None => return,
            Some(recette) => recette,
        };

        let mut ingredients_restant = next_recette.get_ingredients().clone().into_iter().collect::<HashSet<_>>();
        for ingredient in &self.assiette {
            ingredients_restant.remove(ingredient);
        }

        match self.player.get_object_held() {
            None => {
                
            },
            Some(_ingr) => {

            },
        };
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
