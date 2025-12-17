use crate::config::{Category, CommandItem};
use ratatui::widgets::ListState;

pub enum MenuItem {
    CategoryHeader(String),
    Command(CommandItem),
}

pub struct App {
    pub items: Vec<MenuItem>,
    pub state: ListState,
    pub title: String,
}

impl App {
    pub fn new(categories: Vec<Category>, title: String) -> App {
        let mut state = ListState::default();

        // Convert categories into flat list with headers
        let mut items = Vec::new();
        for category in categories {
            items.push(MenuItem::CategoryHeader(category.name));
            for command in category.commands {
                items.push(MenuItem::Command(command));
            }
        }

        // Select first command (skip header)
        state.select(Some(1));

        App {
            items,
            state,
            title,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    1 // Skip first header
                } else {
                    i + 1
                }
            }
            None => 1,
        };

        // Skip category headers
        if matches!(self.items.get(i), Some(MenuItem::CategoryHeader(_))) {
            if i + 1 < self.items.len() {
                self.state.select(Some(i + 1));
            } else {
                self.state.select(Some(1));
            }
        } else {
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 1 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 1,
        };

        // Skip category headers
        if matches!(self.items.get(i), Some(MenuItem::CategoryHeader(_))) {
            if i > 1 {
                self.state.select(Some(i - 1));
            } else {
                self.state.select(Some(self.items.len() - 1));
            }
        } else {
            self.state.select(Some(i));
        }
    }

    pub fn get_selected_command(&self) -> Option<&String> {
        if let Some(i) = self.state.selected() {
            if let Some(MenuItem::Command(cmd)) = self.items.get(i) {
                return Some(&cmd.command);
            }
        }
        None
    }
}
