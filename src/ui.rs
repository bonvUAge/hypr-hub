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

use crate::app::{App, MenuItem};
use crate::commands::execute_command;

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            render_ui(f, app);
        })?;

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

fn render_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|item| match item {
            MenuItem::CategoryHeader(name) => ListItem::new(format!("═══ {} ═══", name)).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            MenuItem::Command(cmd) => ListItem::new(format!("  {}", cmd.name)),
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.title)),
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

fn handle_command_execution(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    if let Some(command_str) = app.get_selected_command() {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        execute_command(command_str)?;

        println!("\n[Done] Press Enter to return to menu...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        enable_raw_mode()?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        terminal.clear()?;
    }

    Ok(())
}
