use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
pub struct Terminal {}

pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

pub struct MoveTerminal {
    pub x: u16,
    pub y: u16,
}

impl TerminalSize {
    pub fn new() -> Self {
        let size = size().unwrap();
        TerminalSize {
            width: size.0,
            height: size.1,
        }
    }
}

impl MoveTerminal {
    pub fn new() -> Self {
        MoveTerminal { x: 0, y: 0 }
    }
}
impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        queue!(stdout(), Clear(ClearType::All))?;
        Self::move_cursor_to(MoveTerminal::new())?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(move_terminal: MoveTerminal) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(move_terminal.x, move_terminal.y))?;
        Ok(())
    }
    pub fn size() -> TerminalSize {
        TerminalSize::new()
    }
}
