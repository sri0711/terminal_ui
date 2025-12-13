use std::io;
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use dotenvy::dotenv;
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
