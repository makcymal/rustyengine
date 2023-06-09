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

/// Clears, enables raw mode anr returns console size as (rows, cols)
pub fn init() -> ReRes<(u16, u16)> {
    let size = size()?;
    enable_raw_mode()?;
    // clear();
    Ok((size.1, size.0))
}

/// Clears console
pub fn clear() {
    print!("\x1b[2J");
}

/// Blocks process until new event obtained
pub fn listen() -> Result<Event> {
    crossterm_event::read()
}

/// Moves cursor on the given position retunring error if position isn't valid
pub(crate) fn move_cursor(row: u16, col: u16) -> Result<()> {
    stdout().execute(cursor::MoveTo(col, row))?;
    Ok(())
}
