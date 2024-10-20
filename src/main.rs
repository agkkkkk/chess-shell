use chess_shell::{
    constants::home_dir,
    game::{Game, GameResult},
    tui::Tui,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    engine_path: String,
}

fn main() -> GameResult<()> {
    // let arg = Args::parse();

    // let home_dir = home_dir()?;
    // let folder_path = home_dir.join("./config/chess-shell");
    // let config_path = home_dir.join(".config/chess-shell/config.toml");

    // Initialize Terminal
    let terminal = ratatui::try_init()?;

    let mut tui = Tui::new(terminal);

    tui.draw()?;

    Ok(())
}
