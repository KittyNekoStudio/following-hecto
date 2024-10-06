use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, Show},
    event::{
        read,
        Event::{self, Key},
        KeyCode::Char,
        KeyEvent, KeyModifiers,
    },
    queue,
    style::Print,
};

use terminal::{MoveTerminal, Terminal};
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
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            stdout().flush()?;
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
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        if self.should_quit {
            Terminal::clear_screen()?;
            queue!(stdout(), Print("Goodbye\r\n"))?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(MoveTerminal::new())?;
        }
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size();
        for current_row in 0..height.height {
            queue!(stdout(), Print("~"))?;
            if current_row + 1 < height.height {
                queue!(stdout(), Print("\r\n"))?;
            }
        }
        Ok(())
    }
}
