use crate::{
    objets::{Case, Direction, Ingredient, IngredientType, Recette},
    player::Player
};
use std::fmt::Display;


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
            vec![Case::Ingredient(pain.clone()), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::COUPER, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None), Case::Vide, Case::Vide, Case::Table(None)],
            vec![Case::Table(None), Case::Ingredient(tomate.clone()), Case::Table(None),Case::Ingredient(salade.clone()),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None),Case::Table(None) ,Case::Table(None),Case::Ingredient(oignon.clone()),Case::Table(None),Case::Table(None),]        ];
        
        Self {
            player: Player::new((1, 1)),
            map,
            recettes: Vec::new(),
            assiette: Vec::new(),
            score: 0,
            t: 0,
            max_t,
            next_recette: rand::random_range(25..=50),
        }
    }


    
    pub fn move_player(&mut self, direction: Direction) {
        self.player.set_facing(direction);
        
        let mut wanted_pos: (usize, usize) = self.get_facing().0;

        match direction {
            Direction::North => {
                if wanted_pos.1 == 0 {
                    return;
                }
                wanted_pos.1 = wanted_pos.1 - 1
            },
            Direction::West => {
                if wanted_pos.0 == 0 {
                    return;
                }
                wanted_pos.0 = wanted_pos.0 - 1
            },
            Direction::South => {
                if wanted_pos.1 == self.map.len()-1 {
                    return;
                }
                wanted_pos.1 = wanted_pos.1 + 1
            },
            Direction::East => {
                if wanted_pos.0 == self.map[0].len() {
                    return;
                }
                wanted_pos.0 = wanted_pos.0 + 1
            },
        }

        if self.map[wanted_pos.1][wanted_pos.0] == Case::Vide  {
                self.player.set_pos(wanted_pos.0, wanted_pos.1, direction);
        }
        
    }

    pub fn get_facing(&self) -> ((usize, usize), Case) {
        let pos: (usize, usize) = self.player.get_pos();
        let mut facing_pos: (usize, usize) = (0, 0);
        let lenx: usize = self.map[0].len();
        let leny: usize = self.map.len();

        match self.player.get_facing() {
            Direction::North => facing_pos.1 = pos.1 - 1,
            Direction::West => facing_pos.0 = pos.1 - 1,
            Direction::South => facing_pos.1 = pos.1 + 1,
            Direction::East => facing_pos.0 = pos.1 + 1,
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
                        self.player.set_object_held(Some(object));
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

        if self.t == self.next_recette {
            self.recettes.push(Recette::new());
            self.next_recette = rand::random_range(25..=50);
        }        
    }
}

// impl Display for Game {
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
