use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenvy::dotenv;
use std::io;
use terminal_app::app::start;

fn main() -> Result<(), io::Error> {
    dotenv().ok();
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    start::init();
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
