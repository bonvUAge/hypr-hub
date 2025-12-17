mod app;
mod commands;
mod config;
mod ui;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Result};

use app::App;
use config::Config;
use ui::run_app;

fn main() -> Result<()> {
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            println!("⚠️  Using default config...");
            Config::default()
        }
    };

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(config.commands, config.app.title);

    let res = run_app(&mut terminal, &mut app);

    // Cleanup and restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("❌ Error: {:?}", err);
    }

    Ok(())
}
