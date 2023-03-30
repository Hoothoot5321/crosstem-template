pub mod basic_cursor_controller;
pub mod basic_line_controller;
pub mod colour_holder;
pub mod cursor_controller;
pub mod lines_controller;
pub mod reader;
pub mod renderer;
pub mod supabase;
pub mod traits;

use std::io::{self, Stdout};

use basic_cursor_controller::BaseCusorController;
use basic_line_controller::BaseLineController;
use colour_holder::ColourHolder;
use crossterm::{cursor, queue, style::Color, terminal};
use cursor_controller::CursorController;
use lines_controller::LinesController;
use reader::Reader;
use renderer::Renderer;
use traits::LineC;
#[tokio::main]
async fn main() -> Result<(), exitfailure::ExitFailure> {
    let content: Vec<String> = vec!["Fiskemanden", "Suiii", "LOLOLOL"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let content2: Vec<String> = vec!["Fiskemanden", "Suiii", "LOLOLOL"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let mut lines_controller = LinesController::new(
        content,
        vec!["Fisk".to_string(), "It is working suiii".to_string()],
    );
    let renderer = Renderer::new();
    let reader = Reader::new();
    let mut cursor_controller = CursorController::new(lines_controller.get_header().len(), 0);

    let mut base_line = BaseLineController::new(content2, vec![]);
    let mut base_cursor = BaseCusorController::new(base_line.get_header().len());

    let mut stdout = io::stdout();

    let mut saki = true;

    let colour_holder = ColourHolder {
        foreground_colour: Color::White,
        background_colour: Color::Black,
        highlight_colour: Color::Blue,
    };

    setup(&mut stdout)?;
    while saki {
        renderer.render_all(&mut stdout, &base_line, &colour_holder, &base_cursor)?;
        saki = reader.read(&mut base_line, &mut base_cursor)?;
    }
    quit(&mut stdout)?;
    Ok(())
}

fn setup(stdout: &mut Stdout) -> Result<(), exitfailure::ExitFailure> {
    terminal::enable_raw_mode()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

fn quit(stdout: &mut Stdout) -> Result<(), exitfailure::ExitFailure> {
    terminal::disable_raw_mode()?;
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;
    Ok(())
}
