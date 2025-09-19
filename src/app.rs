
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use color_eyre::Result;
use crate::game::Game;
use crate::objets::{Direction, Case};

pub struct App {
    pub title: String,
    pub status: String,
    pub left_panel_title: String,
    pub right_panel_title: String,
    pub right_panel_content: String,
    pub should_quit: bool,
    pub game: Game,
}

impl App {
    pub fn new() -> Self {
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
        
        let app = Self {
            title: "Overcook Dark Radé".to_string(),
            status: "Utilisez les flèches pour vous déplacer".to_string(),
            left_panel_title: "Map".to_string(),
            right_panel_title: "Infos".to_string(),
            right_panel_content: "Personnage: @\nMurs: #\nSol: .\n\nUtilisez les flèches\npour vous déplacer!".to_string(),
            should_quit: false,
            game: Game::new(300),
        };
        app
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

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Percentage(67), Percentage(33)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        // Barre de titre
        frame.render_widget(Block::bordered().title(self.title.as_str()), title_area);

        // Barre de statut
        frame.render_widget(Block::bordered().title(self.status.as_str()), status_area);

        // Panneau gauche - bordure principale
        frame.render_widget(
            Block::bordered()
                .title(self.left_panel_title.as_str())
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

        let player_pos = self.game.get_player().get_pos();
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
                    (Style::default().bg(Color::Green).fg(Color::Black), '@')
                } 
                else {match cell {
                    Case::Table(None) => (Style::default().bg(Color::Rgb(142, 73, 26)).fg(Color::White), 'T'),
                    Case::Table(Some(ingr)) => (Style::default().bg(Color::Rgb(142, 73, 26)).fg(Color::White), ingr.char()),
                    Case::Ingredient(ingr) => (Style::default().bg(Color::Red).fg(Color::White), ingr.char()),
                    Case::COUPER => (Style::default().bg(Color::LightBlue).fg(Color::Black), 'C'),
                    Case::ASSIETTE => (Style::default().bg(Color::White).fg(Color::Black), 'A'),
                    _ => (Style::default().bg(Color::Black).fg(Color::White), ' '),
                }};

                // Créer un bloc pour cette cellule
                let cell_block = Block::default().style(style);
                frame.render_widget(cell_block, cell_area);

                // Optionnel : afficher le caractère au centre du bloc
                if cell_width >= 3 && cell_height >= 1 {
                    let text_area = Rect {
                        x: cell_area.x + cell_width / 2,
                        y: cell_area.y + cell_height / 2,
                        width: 1,
                        height: 1,
                    };
                    let cell_paragraph = Paragraph::new(letter.to_string()).style(style);
                    frame.render_widget(cell_paragraph, text_area);
                }
            }
        }
        
        // Panneau droit avec contenu
        let right_paragraph = Paragraph::new(self.right_panel_content.as_str()).block(
            Block::bordered()
                .title(self.right_panel_title.as_str())
                .style(Style::default().bg(Color::Red)),
        );
        frame.render_widget(right_paragraph, right_area);
    }


    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('p') => {
                    self.title = "Nouveau Titre !".to_string();
                }
                KeyCode::Char('s') => {
                    self.status = "Statut modifié !".to_string();
                }
                KeyCode::Char('l') => {
                    self.left_panel_title = "Panneau Gauche Modifié".to_string();
                }
                KeyCode::Char('r') => {
                    self.right_panel_title = "Panneau Droit Modifié".to_string();
                }
                KeyCode::Up => self.game.move_player(Direction::North),
                KeyCode::Down => self.game.move_player(Direction::South),
                KeyCode::Left => self.game.move_player(Direction::West),
                KeyCode::Right => self.game.move_player(Direction::East),
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
