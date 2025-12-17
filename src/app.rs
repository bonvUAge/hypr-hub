// 🦀 Модуль с логикой приложения (App state)
// Это как состояние (state) в React - храним данные и методы для их изменения

use crate::config::CommandItem;
use ratatui::widgets::ListState;

// 🦀 Структура App - это "состояние" нашего приложения
// ВАЖНО: больше нет lifetime 'a, потому что CommandItem - owned данные (String вместо &str)
pub struct App {
    pub items: Vec<CommandItem>, // Теперь Vec<CommandItem> вместо Vec<(&str, &str)>
    pub state: ListState,
    pub title: String, // Заголовок из конфига
}

impl App {
    // 🦀 Конструктор теперь принимает команды и заголовок
    pub fn new(commands: Vec<CommandItem>, title: String) -> App {
        let mut state = ListState::default();
        state.select(Some(0));

        App {
            items: commands,
            state,
            title,
        }
    }

    // 🦀 Переход к следующему пункту меню
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

    // 🦀 Переход к предыдущему пункту меню
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

    // 🦀 Получить выбранную команду
    // Теперь возвращаем &String вместо &str
    pub fn get_selected_command(&self) -> Option<&String> {
        self.state.selected().map(|i| &self.items[i].command)
    }
}
