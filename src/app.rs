use crate::game::Game;
use crate::objets::{ActionResult, Case, Direction, Ingredient};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

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

pub struct App {
    pub right_panel_content: String,
    pub should_quit: bool,
    pub game: Game,
    pub logs: Vec<String>,
    pub ingr_held: Option<Ingredient>,
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
            right_panel_content:
                "Personnage: @\nMurs: #\nSol: .\n\nUtilisez les flèches\npour vous déplacer!"
                    .to_string(),
            should_quit: false,
            game: Game::new(300),
            logs: vec![
                "Application démarrée".to_string(),
                "Carte générée".to_string(),
            ],
            ingr_held: None,
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

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            self.game.update();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            if self.should_quit {
                break Ok(());
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        use Constraint::{Length, Min, Percentage};

        let player_pos = self.game.get_player().get_pos();
        let right_panel_content = format!(
            "Coords perso: x={}, y={}\n\nLégende:\n🧑‍🍳 Joueur\n🪑 Table\n🔪 Station de coupe\n🍽️ Assiette\n🍞 Pain\n🥬 Salade\n🍅 Tomate\n🧅 Oignon\n\nUtilisez les flèches\npour vous déplacer!\nItem actuellement tenu: {}",
            player_pos.0, player_pos.1,
            self.ingr_held.as_ref().map(|ingr| ingr.emoji()).unwrap_or("Aucun")
        );

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Percentage(67), Percentage(33)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        // Diviser le panneau droit en deux : infos (en haut) et logs (en bas)
        let right_vertical = Layout::vertical([Percentage(60), Percentage(40)]);
        let [right_info_area, right_log_area] = right_vertical.areas(right_area);

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
                .style(Style::default().bg(Color::Blue)),
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
                        Case::Table(None) => (
                            Style::default()
                                .bg(Color::Rgb(142, 73, 26))
                                .fg(Color::White),
                            " ",
                        ),
                        Case::Table(Some(ingr)) => (
                            Style::default()
                                .bg(Color::Rgb(142, 73, 26))
                                .fg(Color::White),
                            ingr.emoji(),
                        ),
                        Case::Ingredient(ingr) => (
                            Style::default().bg(Color::Red).fg(Color::White),
                            ingr.emoji(),
                        ),
                        Case::COUPER => {
                            (Style::default().bg(Color::LightBlue).fg(Color::Black), "🔪")
                        }
                        Case::ASSIETTE => (Style::default().bg(Color::White).fg(Color::White), "🍽️"),
                        _ => (Style::default().bg(Color::Black).fg(Color::White), " "),
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
                .style(Style::default().bg(Color::Red)),
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
                .style(Style::default().bg(Color::Yellow)),
        );
        frame.render_widget(log_paragraph, right_log_area);
        
    }

    fn handle_events(&mut self) -> Result<()> {
        let player_pos = self.game.get_player().get_pos();
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => {
                    self.log_fmt("Quitter le jeu");
                    self.should_quit = true;
                }
                KeyCode::Up => {
                    let result = self.game.move_player(Direction::North);
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Déplacement vers le haut réussi, x: {}, y: {}", player_pos.0, player_pos.1);
                        },
                        ActionResult::Blocked => {
                            app_println!(self, "Chemin bloqué vers le haut");
                        },
                        ActionResult::InvalidPosition => {
                            app_println!(self, "Position invalide");
                        },
                        _ => {}
                    }
                }
                KeyCode::Down => {
                    let result = self.game.move_player(Direction::South);
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Déplacement vers le bas réussi, x: {}, y: {}", player_pos.0, player_pos.1);
                        },
                        ActionResult::Blocked => {
                            app_println!(self, "Chemin bloqué vers le bas");
                        },
                        ActionResult::InvalidPosition => {
                            app_println!(self, "Position invalide");
                        },
                        _ => {}
                    }
                }
                KeyCode::Left => {
                    let result = self.game.move_player(Direction::West);
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Déplacement vers la gauche réussi, x: {}, y: {}", player_pos.0, player_pos.1);
                        },
                        ActionResult::Blocked => {
                            app_println!(self, "Chemin bloqué vers la gauche");
                        },
                        ActionResult::InvalidPosition => {
                            app_println!(self, "Position invalide");
                        },
                        _ => {}
                    }
                }
                KeyCode::Right => {
                    let result = self.game.move_player(Direction::East);
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Déplacement vers la droite réussi, x: {}, y: {}", player_pos.0, player_pos.1);
                        },
                        ActionResult::Blocked => {
                            app_println!(self, "Chemin bloqué vers la droite");
                        },
                        ActionResult::InvalidPosition => {
                            app_println!(self, "Position invalide");
                        },
                        _ => {}
                    }
                }
                KeyCode::Char('p') => {
                    let result = self.game.pickup();
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Objet ramassé avec succès");
                        },
                        ActionResult::HandsFull => {
                            app_println!(self, "Mains pleines ! Impossible de ramasser");
                        },
                        ActionResult::NoTarget => {
                            app_println!(self, "Rien à ramasser ici");
                        },
                        _ => {}
                    }
                }
                KeyCode::Char('d') => {
                    let result = self.game.deposit();
                    match result {
                        ActionResult::Success => {
                            app_println!(self, "Objet déposé avec succès");
                        },
                        ActionResult::HandsEmpty => {
                            app_println!(self, "Mains vides ! Rien à déposer");
                        },
                        ActionResult::TableOccupied => {
                            app_println!(self, "Table occupée ! Impossible de déposer");
                        },
                        ActionResult::NoTarget => {
                            app_println!(self, "Impossible de déposer ici");
                        },
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
