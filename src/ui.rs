// 🦀 Модуль UI - отрисовка интерфейса и обработка событий

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};
use std::io::Result;

use crate::app::App;
use crate::commands::execute_command;

// 🦀 Главный цикл приложения
pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        // 🦀 Отрисовка UI
        terminal.draw(|f| {
            render_ui(f, app);
        })?;

        // 🦀 Обработка событий клавиатуры
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => {
                    handle_command_execution(terminal, app)?;
                }
                _ => {}
            }
        }
    }
}

// 🦀 Функция отрисовки UI
fn render_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // 🦀 Создаём список элементов
    // ВАЖНО: теперь используем item.name.as_str() потому что name это String
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|item| ListItem::new(item.name.as_str()))
        .collect();

    // 🦀 Создаём виджет списка с заголовком из конфига
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.title)), // Заголовок из конфига!
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, size, &mut app.state);
}

// 🦀 Обработка выполнения команды
fn handle_command_execution(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    if let Some(command_str) = app.get_selected_command() {
        // 1. Выходим из raw mode
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        // 2. Запускаем команду
        execute_command(command_str)?;

        // 3. Ждём нажатия Enter
        println!("\n[Done] Press Enter to return to menu...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // 4. Возвращаемся в raw mode
        enable_raw_mode()?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        terminal.clear()?;
    }

    Ok(())
}
