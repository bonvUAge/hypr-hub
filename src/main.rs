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
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io::{self, Result};
use std::process::{Command, Stdio}; // Нужно для запуска команд

// 🦀 СТРУКТУРА ПРИЛОЖЕНИЯ
struct App<'a> {
    // Список пар: (Название в меню, Команда для консоли)
    items: Vec<(&'a str, &'a str)>,
    state: ListState,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut state = ListState::default();
        state.select(Some(0)); // Выбираем первый элемент при старте

        App {
            items: vec![
                ("🔂 Update System", "./scripts/update.sh"),
                ("🧹 Clean Pacman&Paru Cache", "./scripts/scc.sh"),
                ("🪠 Clean RAM", "sudo sync; sudo sysctl -w vm.drop_caches=3"),
            ],
            state,
        }
    }

    // Переход к следующему пункту (циклический)
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Переход к предыдущему пункту (циклический)
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<()> {
    // 🦀 1. enable_raw_mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // declare App
    let mut app = App::new();

    // run app
    let res = run_app(&mut terminal, &mut app);

    // 🦀 2. disable_raw_mode
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        // draw area
        terminal.draw(|f| {
            let size = f.area();

            // create list items
            let items: Vec<ListItem> = app.items.iter().map(|i| ListItem::new(i.0)).collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Main system HUB 🦀 "),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, size, &mut app.state);
        })?;

        // Key commands
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),

                // Enter
                KeyCode::Enter => {
                    if let Some(selected_index) = app.state.selected() {
                        let command_str = app.items[selected_index].1;

                        // 1. disable_raw_mode()
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                        // 2. launch script
                        println!("Executing: {}", command_str);

                        // sh-c
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(command_str)
                            .status()?;

                        println!("\n[Done] Press Enter to return to menu...");

                        // 3. wait for Enter from user
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;

                        // 4. enable_raw_mode()
                        enable_raw_mode()?;
                        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                        terminal.clear()?;
                    }
                }
                _ => {}
            }
        }
    }
}
