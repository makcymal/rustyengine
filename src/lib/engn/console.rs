use {
    crossterm::{
        cursor, event as crossterm_event,
        terminal::{enable_raw_mode, size},
        ExecutableCommand,
        Result,
    },
    std::{
        io::{
            stdout, Error as IoError,
        }
    },
    crate::errs::{
        ReRes,
        ReErr
    },
};

pub use crossterm::event::Event;

// (columns, rows)
pub fn init() -> ReRes<(u16, u16)> {
    let size = size()?;
    enable_raw_mode()?;
    clear();
    Ok((size.0, size.1))
}

pub fn clear() {
    print!("\x1b[2J");
}

pub fn listen() -> Result<Event> {
    crossterm_event::read()
}

pub(crate) fn move_cursor(row: u16, col: u16) -> Result<()> {
    stdout().execute(cursor::MoveTo(col, row))?;
    Ok(())
}
