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
//     let mut game = HAI716I_POASMA::game::Game::new(3000000);
//     loop {
//         game.robot();
//         game.update();
//         // print!("\x1B[2J\x1B[1;1H");
//         // println!("MAP:\n{}", game);
//         // std::thread::sleep(std::time::Duration::from_millis(200));
//     }
// }