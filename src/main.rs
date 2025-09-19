use HAI716I_POASMA::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
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