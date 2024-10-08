use std::io::Error;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};

use terminal::{Position, Size, Terminal};
mod terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::print_name()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn print_name() -> Result<(), Error> {
        let size = Terminal::size()?;

        Terminal::move_cursor_to(Position {
            x: size.width / 2,
            y: size.height / 2,
        })?;

        Terminal::print("Hecto")?;
        Terminal::move_cursor_to(Position {
            x: size.width / 2 - 3,
            y: size.height / 2 + 1,
        })?;
        Terminal::print("Version 0.0.1")?;

        Ok(())
    }
}
