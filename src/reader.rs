use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::traits::{CursorC, LineC};

pub struct Reader {}

impl Reader {
    pub fn new() -> Reader {
        Reader {}
    }
    fn read_quit(&self, event: KeyEvent) -> Result<bool, exitfailure::ExitFailure> {
        return match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::ALT,
                ..
            } => Ok(false),
            _ => Ok(true),
        };
    }

    pub fn read<L: LineC, C: CursorC>(
        &self,
        lines_controller: &mut L,
        cursor_controller: &mut C,
    ) -> Result<bool, exitfailure::ExitFailure> {
        let mut return_bool = true;
        if let Ok(Event::Key(event)) = event::read() {
            return_bool = self.read_quit(event)?;
            cursor_controller.take_input(event, lines_controller.get_lines());
            lines_controller.take_input(event, cursor_controller);
        }
        Ok(return_bool)
    }
}
