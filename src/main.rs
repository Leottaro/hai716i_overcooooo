use color_eyre::Result;
use hai716i_poasma::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    Ok(result?)
}

// fn main() {
//     let mut game = Game::new();
//     loop {
//         game.update();
//         print!("\x1B[2J\x1B[1;1H");
//         println!("{}", game);
//         std::thread::sleep(std::time::Duration::from_millis(200));
//     }
// }
