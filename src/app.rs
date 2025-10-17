use crate::game::{DepositError, Game, PickupError};
use crate::objets::Case;
use crate::{APP_TITLE, ROBOT_COOLDOWN};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::prelude::Backend;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Gauge, Paragraph},
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

fn percent_to_color(percent: f32) -> Color {
    if percent > 0.5 {
        Color::Green
    } else if percent > 0.2 {
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
            game: Game::new(),
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

    pub fn reset_game(&mut self) {
        self.game = Game::new();
        self.logs.clear();
        self.should_quit = false;
        app_println!(self, "Partie réinitialisée");
    }

    pub fn log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn log_fmt(&mut self, message: &str) {
        self.log(message.to_string());
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>, robot: bool) -> io::Result<()> {
        let mut next_robot = Instant::now();

        loop {
            // Gérer les événements avec timeout
            if event::poll(Duration::from_millis(16))? {
                let return_handle = self.handle_events(robot);
                if let Err(e) = return_handle {
                    self.log_fmt(&format!("Erreur event: {}", e));
                }
            }

            if self.should_quit {
                return Ok(());
            }

            if !self.game.is_finished() {
                // Vérifier si c'est le moment de faire un tick
                let now = Instant::now();
                if robot && next_robot < now {
                    self.game.robot();
                    next_robot = now + ROBOT_COOLDOWN;
                }
                self.game.tick(now);

                // Render UI
                terminal.draw(|frame| self.draw(frame))?;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        use Constraint::{Length, Min, Percentage};

        let player = self.game.get_player();
        let right_panel_content = format!(
            "Utilisez les flèches pour vous déplacer! \nItem en main: {} \nPosition: {:?} \nDirection : {} \nAssiette: {} \nScore: {}\n",
            self.game
                .get_player()
                .get_object_held()
                .map_or("Rien".to_string(), |ingr| ingr.emoji().to_string()),
            player.get_pos(),
            player.get_facing().emoji(),
            self.game
                .get_assiette()
                .iter()
                .map(|ingr| ingr.emoji())
                .collect::<Vec<_>>()
                .join(", "),
            self.game.get_score(),
        );

        let vertical = Layout::vertical([Length(1), Min(0), Length(5)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());

        let gauge = Gauge::default()
            .percent((self.game.get_percent_left() * 100.) as u16)
            .label(format!(
                "Temps restant: {:.2}s",
                self.game.get_remaining_time().as_secs_f32()
            ))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .gauge_style(
                Style::default()
                    .bg(Color::Black)
                    .fg(percent_to_color(self.game.get_percent_left())),
            );
        frame.render_widget(gauge, status_area);
        let horizontal = Layout::horizontal([Percentage(67), Percentage(33)]);
        let [left_area, right_area] = horizontal.areas(main_area);

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
        let padded_recipe_list = right_recipe_list.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Blue)),
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

            let recipe_box = Block::bordered()
                .title(format!("Recette {}", i + 1))
                .style(Style::default().bg(percent_to_color(recette.get_percent_left())));

            let area = Rect {
                x: padded_recipe_list.x,
                y: padded_recipe_list.y + (i * recette_height) as u16,
                width: padded_recipe_list.width,
                height: recette_height as u16,
            };

            frame.render_widget(recipe_box, area);

            let [para_area, gauge_area] = Layout::vertical([Min(1), Length(1)]).areas(area);

            let para_area_padded = para_area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            });
            let recipe_paragraph =
                Paragraph::new(format!("Ingrédients : {}\nTemps restant :", ingredients));
            frame.render_widget(recipe_paragraph, para_area_padded);

            let gauge_area_padded = gauge_area.inner(Margin {
                vertical: 0,
                horizontal: 1,
            });
            let gauge = Gauge::default()
                .percent((recette.get_percent_left() * 100.) as u16)
                .label(format!("{:.2}s", recette.get_temps_restant().as_secs_f32()))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .gauge_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(percent_to_color(recette.get_percent_left())),
                );
            frame.render_widget(gauge, gauge_area_padded);
        }
        frame.render_widget(Block::bordered().title(APP_TITLE), title_area);

        frame.render_widget(Block::bordered(), status_area);

        frame.render_widget(
            Block::bordered()
                .title("Game")
                .style(Style::default().bg(Color::Black)),
            left_area,
        );

        let inner_area = Rect {
            x: left_area.x + 1,
            y: left_area.y + 1,
            width: left_area.width.saturating_sub(2),
            height: left_area.height.saturating_sub(2),
        };

        let map_width = self.game.get_map()[0].len() as u16;
        let map_height = self.game.get_map().len() as u16;
        let cell_width = inner_area.width / map_width;
        let cell_height = inner_area.height / map_height;

        for (y, row) in self.game.get_map().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_area = Rect {
                    x: inner_area.x + (x as u16) * cell_width,
                    y: inner_area.y + (y as u16) * cell_height,
                    width: cell_width,
                    height: cell_height,
                };

                let (style, letter) = if (x, y) == player.get_pos() {
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
                        Case::ASSIETTE => {
                            (Style::default().bg(Color::DarkGray).fg(Color::White), "🍽️")
                        }
                        _ => (Style::default().bg(Color::White).fg(Color::White), " "),
                    }
                };

                let cell_block = Block::default().style(style);
                frame.render_widget(cell_block, cell_area);

                if cell_width >= 2 && cell_height >= 1 {
                    let text_area = Rect {
                        x: cell_area.x + cell_width / 2,
                        y: cell_area.y + cell_height / 2,
                        width: 2,
                        height: 1,
                    };
                    let cell_paragraph = Paragraph::new(letter).style(style);
                    frame.render_widget(cell_paragraph, text_area);
                }
            }
        }

        let right_paragraph = Paragraph::new(right_panel_content.as_str()).block(
            Block::bordered()
                .title("Infos")
                .style(Style::default().bg(Color::Blue)),
        );
        frame.render_widget(right_paragraph, right_info_area);

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

        // Si perdu, afficher une boîte modale centrée "Game Over"
        if self.game.is_finished() {
            use ratatui::widgets::Clear;
            let area = frame.area();
            let width = std::cmp::min(40, area.width.saturating_sub(10));
            let height = 7u16;
            let x = area.x + (area.width.saturating_sub(width)) / 2;
            let y = area.y + (area.height.saturating_sub(height)) / 2;
            let rect = Rect {
                x,
                y,
                width,
                height,
            };

            // effacer l'arrière-plan de la zone et dessiner la boîte
            frame.render_widget(Clear, rect);
            let block = Block::bordered()
                .title("Game Over")
                .style(Style::default().bg(Color::Black).fg(Color::LightRed));
            frame.render_widget(block, rect);

            let inner = rect.inner(Margin {
                vertical: 1,
                horizontal: 2,
            });
            let text = Paragraph::new(
                format!("Partie finie !\nScore final: {}\n\nAppuyez sur R pour rejouer \nou échap pour quitter.", self.game.get_score()),
            )
            .style(Style::default().fg(Color::White));
            frame.render_widget(text, inner);
        }
    }

    fn handle_events(&mut self, robot: bool) -> Result<()> {
        let key_event = match event::read()? {
            Event::Key(key) => key,
            _ => return Ok(()),
        };

        // Accept both Press and Repeat events so keys like 'r' are handled
        // even when the terminal emits Repeat instead of Press.
        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return Ok(());
        }

        let key_code = key_event.code;
        match key_code {
            KeyCode::Esc => {
                self.log_fmt("Quitter le jeu");
                self.should_quit = true;
                return Ok(());
            }
            KeyCode::Char('r') => {
                app_println!(self, "reset !!!");
                self.reset_game();
            }
            _ => {}
        }

        if robot {
            return Ok(());
        }

        match key_code {
            // KeyCode::Up | KeyCode::Char('z') => {
            //     self.game.move_player(Direction::North);
            // }
            // KeyCode::Down | KeyCode::Char('s') => {
            //     self.game.move_player(Direction::South);
            // }
            // KeyCode::Left | KeyCode::Char('q') => {
            //     self.game.move_player(Direction::West);
            // }
            // KeyCode::Right | KeyCode::Char('d') => {
            //     self.game.move_player(Direction::East);
            // }
            KeyCode::Char(' ') => {
                let result = self.game.pickup();
                match result {
                    Ok(()) => app_println!(self, "Objet ramassé avec succès"),
                    Err(PickupError::HandsFull) => {
                        app_println!(self, "Mains pleines ! Impossible de ramasser")
                    }
                    Err(PickupError::AssietteEmpty) => {
                        app_println!(self, "Assiette vide ! Rien à ramasser")
                    }
                    Err(PickupError::TableEmpty) => {
                        app_println!(self, "Table vide ! Rien à ramasser")
                    }
                    Err(PickupError::NoTarget((pos, _))) => {
                        app_println!(self, "Impossible de ramasser à {:?}", pos)
                    }
                }
            }
            KeyCode::Char('e') => {
                let result = self.game.deposit();
                match result {
                    Ok(()) => app_println!(self, "Objet déposé avec succès"),
                    Err(DepositError::HandsEmpty) => {
                        app_println!(self, "Mains vides ! Rien à déposer")
                    }
                    Err(DepositError::TableFull) => {
                        app_println!(self, "Table occupée ! Impossible de déposer")
                    }
                    Err(DepositError::NoTarget((pos, _))) => {
                        app_println!(self, "Impossible de déposer à {:?}", pos)
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
