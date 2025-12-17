// 🦀 Главная точка входа в приложение
// Здесь мы только инициализируем терминал и запускаем приложение

mod app;
mod commands;
mod config;
mod ui; // 🦀 Добавили новый модуль!

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
    // 🦀 Загружаем конфиг
    // В JavaScript это было бы: const config = require('./config.json')
    // Но здесь с обработкой ошибок!
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            println!("⚠️  Использую дефолтный конфиг...");
            Config::default()
        }
    };

    // 🦀 Инициализация терминала
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 🦀 Создаём наше приложение с данными из конфига
    let mut app = App::new(config.commands, config.app.title);

    // 🦀 Запускаем основной цикл
    let res = run_app(&mut terminal, &mut app);

    // 🦀 Очистка и восстановление терминала
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("❌ Error: {:?}", err);
    }

    Ok(())
}
