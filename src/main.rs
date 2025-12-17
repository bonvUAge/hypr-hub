// 🦀 Главная точка входа в приложение
// Здесь мы только инициализируем терминал и запускаем приложение
// Похоже на index.js в мире JavaScript - минимум логики, максимум делегирования!

mod app;
mod ui;
mod commands;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use std::io::{self, Result};

use app::App;
use ui::run_app;

fn main() -> Result<()> {
    // 🦀 Инициализация терминала
    // enable_raw_mode() - это как event.preventDefault() в JavaScript!
    // Мы перехватываем управление терминалом, чтобы самим обрабатывать все события
    enable_raw_mode()?;
    
    let mut stdout = io::stdout();
    // EnterAlternateScreen - переключает на альтернативный буфер экрана
    // Когда выйдем из программы, терминал вернётся к прежнему состоянию!
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 🦀 Создаём наше приложение
    let mut app = App::new();

    // 🦀 Запускаем основной цикл
    let res = run_app(&mut terminal, &mut app);

    // 🦀 Очистка и восстановление терминала
    // Важно делать это ВСЕГДА, даже при ошибке!
    // В JavaScript это похоже на finally {} блок
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Если была ошибка - покажем её
    if let Err(err) = res {
        println!("❌ Ошибка: {:?}", err);
    }

    Ok(())
}
