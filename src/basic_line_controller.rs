use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::traits::{CursorC, LineC};

pub struct BaseLineController {
    pub header: Vec<String>,
    pub lines: Vec<String>,
}
impl BaseLineController {
    pub fn new(lines: Vec<String>, header: Vec<String>) -> BaseLineController {
        BaseLineController { header, lines }
    }
}

impl LineC for BaseLineController {
    fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }
    fn get_header(&self) -> &Vec<String> {
        &self.header
    }
    fn take_input<C: CursorC>(&mut self, event: KeyEvent, cursor_controller: &mut C) {
        match event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => self
                .lines
                .push((&self.lines[cursor_controller.get_y()]).to_string()),
            _ => {}
        }
    }
}
