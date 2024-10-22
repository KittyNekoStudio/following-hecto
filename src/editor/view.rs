use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello World!\r\n")?;

        for current_row in 1..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;

        Ok(())
    }
    //fn hello_message() -> Result<(), Error> {
    //let mut hello_message = format!("Hello World!");
    //let width = Terminal::size()?.width;
    //let len = hello_message.len();
    //#[allow(clippy::integer_division)]
    //let padding = (width.saturating_sub(len)) / 2;
    //let spaces = " ".repeat(padding.saturating_sub(1));
    //hello_message = format!("~{spaces}{hello_message}");
    //hello_message.truncate(width);
    //Terminal::print(&hello_message)?;

    //Ok(())
    //}
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;

        Ok(())
    }
}
