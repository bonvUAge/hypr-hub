// 🦀 Модуль UI - отрисовка интерфейса и обработка событий
// Это как компонент в React - отвечает за визуальное представление

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use std::io::Result;

use crate::app::App;
use crate::commands::execute_command;

// 🦀 Главный цикл приложения
// Похож на game loop или requestAnimationFrame в браузере!
pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        // 🦀 Отрисовка UI
        // terminal.draw() принимает closure (замыкание)
        // Это как callback функция в JavaScript: (f) => { ... }
        terminal.draw(|f| {
            render_ui(f, app);
        })?;

        // 🦀 Обработка событий клавиатуры
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()), // Выход по 'q'
                KeyCode::Down => app.next(),         // Вниз
                KeyCode::Up => app.previous(),       // Вверх
                KeyCode::Enter => {
                    // При нажатии Enter запускаем выбранную команду
                    handle_command_execution(terminal, app)?;
                }
                _ => {} // Игнорируем остальные клавиши
            }
        }
    }
}

// 🦀 Функция отрисовки UI
// Frame - это как canvas контекст в JavaScript
// Мы "рисуем" виджеты на фрейме
fn render_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // 🦀 Создаём список элементов
    // .iter() - это итератор (как Array.map() в JavaScript)
    // .map() - трансформируем каждый элемент
    // .collect() - собираем результат в Vec
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| ListItem::new(i.0))
        .collect();

    // 🦀 Создаём виджет списка
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Main system HUB 🦀 "),
        )
        .highlight_style(
            // Стиль выделенного элемента
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // 🦀 Рендерим stateful виджет
    // Передаём &mut app.state, чтобы виджет знал, что выбрано
    f.render_stateful_widget(list, size, &mut app.state);
}

// 🦀 Обработка выполнения команды
// Здесь мы временно выходим из TUI режима, запускаем команду,
// и возвращаемся обратно
fn handle_command_execution(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    if let Some(command_str) = app.get_selected_command() {
        // 1. Выходим из raw mode и альтернативного экрана
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        // 2. Запускаем команду
        execute_command(command_str)?;

        // 3. Ждём нажатия Enter от пользователя
        println!("\n[Done] Press Enter to return to menu...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // 4. Возвращаемся в raw mode и альтернативный экран
        enable_raw_mode()?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        terminal.clear()?;
    }

    Ok(())
}
