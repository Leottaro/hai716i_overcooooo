use crate::game::Game;
use crate::objets::{ActionResult, Case, Direction};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::prelude::Backend;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Margin},
    style::{Color, Style},
    widgets::{Block, Paragraph, Gauge},
};
use std::io;
use std::time::{Duration, Instant};

const BROWN: Color = Color::Rgb(142, 73, 26);

// Macros pour rediriger les prints vers le système de log
#[macro_export]
macro_rules! app_print {
    ($app:expr, $($arg:tt)*) => {
        $app.log_fmt(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! app_println {
    ($app:expr, $($arg:tt)*) => {
        $app.log_fmt(&format!($($arg)*))
    };
}

fn time_to_color(temps_restant: u32, temps_initial: u32) -> Color {
    let ratio = temps_restant as f32 / temps_initial as f32;
    if ratio > 0.5 {
        Color::Green
    } else if ratio > 0.2 {
        Color::Yellow
    } else {
        Color::Red
    }
}

pub struct App {
    pub right_panel_content: String,
    pub should_quit: bool,
    pub game: Game,
    pub logs: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut map = vec![vec![".".to_string(); 20]; 15];

        // Ajouter des murs autour de la map
        for x in 0..20 {
            map[0][x] = "#".to_string();
            map[14][x] = "#".to_string();
        }
        for row in map.iter_mut() {
            row[0] = "#".to_string();
            row[19] = "#".to_string();
        }

        Self {
            right_panel_content: "".to_string(),
            should_quit: false,
            game: Game::new(300),
            logs: vec![
                "Application démarrée".to_string(),
                "Carte générée".to_string(),
            ],
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&mut self, message: String) {
        self.logs.push(message);
        // Garder seulement les 100 derniers messages pour éviter la surcharge mémoire
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    // Fonction utilitaire pour logger avec formatage
    pub fn log_fmt(&mut self, message: &str) {
        self.log(message.to_string());
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let tick_rate = Duration::from_millis(250); // 4 fois par seconde
        let mut last_tick = Instant::now();

        loop {
            // Render UI
            terminal.draw(|frame| self.draw(frame))?;

            // Gérer le timing
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // Gérer les événements avec timeout
            if event::poll(timeout)? {
                let return_handle = self.handle_events();
                if let Err(e) = return_handle {
                    self.log_fmt(&format!("Erreur event: {}", e));
                }
            }

            // Vérifier si c'est le moment de faire un tick
            if last_tick.elapsed() >= tick_rate {
                self.game.tick();
                last_tick = Instant::now();
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        use Constraint::{Length, Min, Percentage};

        let player_pos = self.game.get_player().get_pos();
        let right_panel_content = format!(
            "Coords perso: x={}, y={}\n\nLégende:\n🧑‍🍳 Joueur\n🪑 Table\n🔪 Station de coupe\n🍽️ Assiette\n🍞 Pain\n🥬 Salade\n🍅 Tomate\n🧅 Oignon\n\nUtilisez les flèches pour vous déplacer!\nItem actuellement tenu: {}\nPosition actuelle: ({}, {})\nRegarde vers: {:?}",
            player_pos.0,
            player_pos.1,
            self.game
                .get_player()
                .get_object_held()
                .map_or("Rien".to_string(), |ingr| ingr.emoji().to_string()),
            player_pos.0,
            player_pos.1,
            self.game.get_player().get_facing(),
        );

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Percentage(67), Percentage(33)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        // Diviser le panneau droit en deux : infos (en haut) et logs (en bas)
        let right_vertical = Layout::vertical([Min(17), Percentage(80), Percentage(20)]);
        let [right_info_area, right_recipe_list, right_log_area] = right_vertical.areas(right_area);
        

        // Afficher les recettes dans le panneau des recettes
        frame.render_widget(
            Block::bordered()
                .title(format!("Recettes ({})", self.game.get_recettes().len()))
                .style(Style::default().bg(Color::Blue)),
            right_recipe_list,
        );
        let recettes = self.game.get_recettes();
        let recette_height = 5;
        // Calculer la zone avec padding (par exemple, 1 caractère tout autour)
        let padded_recipe_list = right_recipe_list.inner(Margin { vertical: 1, horizontal: 1 });
        // Utiliser padded_recipe_list pour afficher les blocs de recettes
        frame.render_widget(
            Block::default()
                .style(Style::default().bg(Color::Blue)),
            padded_recipe_list,
        );
        let max_recettes = padded_recipe_list.height as usize / recette_height;
        for (i, recette) in recettes.iter().take(max_recettes).enumerate() {
            let ingredients = recette
                .get_ingredients()
                .iter()
                .map(|ingr| ingr.emoji())
                .collect::<Vec<&str>>()
                .join(", ");

            let recipe_box =
                Block::bordered()
                    .title(format!("Recette {}", i + 1))
                    .style(Style::default().bg(time_to_color(
                        recette.get_temps_restant(),
                        recette.get_temps_initial(),
                    )));

            // Calculer la zone pour chaque recette
            let area = Rect {
                x: padded_recipe_list.x,
                y: padded_recipe_list.y + (i * recette_height) as u16,
                width: padded_recipe_list.width,
                height: recette_height as u16,
            };

            // Dessiner le bloc sur toute la zone
            frame.render_widget(recipe_box, area);

            // Diviser la zone en deux (paragraphe + gauge)
            let [para_area, gauge_area] = Layout::vertical([
                Length(recette_height as u16 - 1),
                Length(1),
            ]).areas(area);

            // Paragraphe avec padding
            let para_area_padded = para_area.inner(Margin { vertical: 1, horizontal: 1 });
            let recipe_paragraph = Paragraph::new(format!(
                "Ingrédients : {}\nTemps restant :",
                ingredients,
            ));
            frame.render_widget(recipe_paragraph, para_area_padded);

            // Gauge avec padding (optionnel, selon le rendu souhaité)
            let gauge_area_padded = gauge_area.inner(Margin { vertical: 0, horizontal: 1 });
            let percent = (recette.get_temps_restant() * 100 / recette.get_temps_initial()).min(100);
            let gauge = Gauge::default()
                .percent(percent as u16)
                .label(format!("{}s", recette.get_temps_restant()))
                .style(Style::default().fg(Color::Green).bg(Color::Black));
            frame.render_widget(gauge, gauge_area_padded);
        }

        // Barre de titre
        frame.render_widget(Block::bordered().title("Overcook Dark Radé"), title_area);

        // Barre de statut
        frame.render_widget(
            Block::bordered().title("Utilisez les flèches pour vous déplacer"),
            status_area,
        );

        // Panneau gauche - bordure principale
        frame.render_widget(
            Block::bordered()
                .title("Game")
                .style(Style::default().bg(Color::Black)),
            left_area,
        );

        // Calculer l'espace disponible à l'intérieur du panneau gauche
        let inner_area = Rect {
            x: left_area.x + 1,
            y: left_area.y + 1,
            width: left_area.width.saturating_sub(2),
            height: left_area.height.saturating_sub(2),
        };

        // Calculer la taille de chaque cellule
        let map_width = self.game.get_map()[0].len() as u16;
        let map_height = self.game.get_map().len() as u16;
        let cell_width = inner_area.width / map_width;
        let cell_height = inner_area.height / map_height;

        // Dessiner chaque cellule de la map comme un bloc
        for (y, row) in self.game.get_map().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_area = Rect {
                    x: inner_area.x + (x as u16) * cell_width,
                    y: inner_area.y + (y as u16) * cell_height,
                    width: cell_width,
                    height: cell_height,
                };

                // Choisir la couleur en fonction du contenu de la cellule

                let (style, letter) = if (x, y) == player_pos {
                    (Style::default().bg(Color::Green).fg(Color::Black), "🧑‍🍳")
                } else {
                    match cell {
                        Case::Table(None) => (Style::default().bg(BROWN).fg(Color::White), " "),
                        Case::Table(Some(ingr)) => {
                            (Style::default().bg(BROWN).fg(Color::White), ingr.emoji())
                        }
                        Case::Ingredient(ingr) => (
                            Style::default().bg(Color::Red).fg(Color::White),
                            ingr.emoji(),
                        ),
                        Case::COUPER => {
                            (Style::default().bg(Color::LightBlue).fg(Color::Black), "🔪")
                        }
                        Case::ASSIETTE => (Style::default().bg(BROWN).fg(Color::White), "🍽️"),
                        _ => (Style::default().bg(Color::White).fg(Color::White), " "),
                    }
                };

                // Créer un bloc pour cette cellule
                let cell_block = Block::default().style(style);
                frame.render_widget(cell_block, cell_area);

                // Afficher l'emoji au centre du bloc
                if cell_width >= 2 && cell_height >= 1 {
                    let text_area = Rect {
                        x: cell_area.x + cell_width / 2,
                        y: cell_area.y + cell_height / 2,
                        width: 2, // Largeur augmentée pour les emojis
                        height: 1,
                    };
                    let cell_paragraph = Paragraph::new(letter).style(style);
                    frame.render_widget(cell_paragraph, text_area);
                }
            }
        }

        // Panneau droit avec contenu des infos (partie supérieure)
        let right_paragraph = Paragraph::new(right_panel_content.as_str()).block(
            Block::bordered()
                .title("Infos")
                .style(Style::default().bg(Color::Blue)),
        );
        frame.render_widget(right_paragraph, right_info_area);

        // Panneau des logs (partie inférieure du panneau droit)
        let log_content = if self.logs.is_empty() {
            "Aucun log pour le moment...".to_string()
        } else {
            let start_index = self.logs.len().saturating_sub(10);
            self.logs[start_index..].join("\n")
        };

        let log_paragraph = Paragraph::new(log_content.as_str()).block(
            Block::bordered()
                .title("Logs")
                .style(Style::default().bg(Color::Blue)),
        );
        frame.render_widget(log_paragraph, right_log_area);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    self.log_fmt("Quitter le jeu");
                    self.should_quit = true;
                }
                KeyCode::Up | KeyCode::Char('z') => {
                    self.game.move_player(Direction::North);
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    self.game.move_player(Direction::South);
                }
                KeyCode::Left | KeyCode::Char('q') => {
                    self.game.move_player(Direction::West);
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    self.game.move_player(Direction::East);
                }
                KeyCode::Char(' ') => {
                    let result = self.game.pickup();
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Objet ramassé avec succès");
                        }
                        ActionResult::HandsFull => {
                            app_println!(self, "Mains pleines ! Impossible de ramasser");
                        }
                        ActionResult::NoTarget => {
                            app_println!(self, "Rien à ramasser à {:?}", self.game.get_facing());
                        }
                        _ => {}
                    }
                }
                KeyCode::Char('e') => {
                    let result = self.game.deposit();
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Objet déposé avec succès");
                        }
                        ActionResult::HandsEmpty => {
                            app_println!(self, "Mains vides ! Rien à déposer");
                        }
                        ActionResult::TableOccupied => {
                            app_println!(self, "Table occupée ! Impossible de déposer");
                        }
                        ActionResult::NoTarget => {
                            app_println!(self, "Impossible de déposer ici");
                        }
                        _ => {}
                    }
                }
                KeyCode::Char('r') => {
                    self.game.ajouter_recette_random();
                    app_println!(self, "Nouvelle recette ajoutée !");
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
