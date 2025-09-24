use color_eyre::Result;
use hai716i_poasma::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal, true);
    ratatui::restore();
    Ok(result?)
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
