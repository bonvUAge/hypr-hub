use crate::config::CommandItem;
use ratatui::widgets::ListState;

pub struct App {
    pub items: Vec<CommandItem>,
    pub state: ListState,
    pub title: String,
}

impl App {
    pub fn new(commands: Vec<CommandItem>, title: String) -> App {
        let mut state = ListState::default();
        state.select(Some(0));

        App {
            items: commands,
            state,
            title,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn get_selected_command(&self) -> Option<&String> {
        self.state.selected().map(|i| &self.items[i].command)
    }
}
