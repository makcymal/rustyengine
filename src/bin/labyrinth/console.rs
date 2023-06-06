use {
    super::action::*,
    anyhow::Result,
    crossterm::{
        cursor, event as crossterm_event,
        terminal::{enable_raw_mode, size},
        ExecutableCommand,
    },
    std::{
        io::stdout, thread, time::Duration,
    },
};

// (columns, rows)
pub fn init() -> Result<(u16, u16)> {
    let size = size()?;
    enable_raw_mode()?;
    clear();
    Ok((size.0, size.1 - 3))
}

pub fn clear() {
    print!("\x1b[2J");
}

pub fn listen() -> Result<EventDiscr> {
    let event = crossterm_event::read()?;
    Ok(EventDiscr::from(event))
}

pub fn show_message(msg: &str, rows: u16, cols: u16) -> Result<()> {
    let col = cols.saturating_sub(msg.len() as u16) / 2;
    let row = rows / 2;
    move_cursor(row, col)?;

    clear();
    println!("{}", &msg[..(cols as usize)]);
    thread::sleep(Duration::from_millis(500));
    clear();
    Ok(())
}

pub fn show_notification(ntf: &str, cols: u16) -> Result<()> {
    let col = cols.saturating_sub(ntf.len() as u16) / 2;
    move_cursor(1, col)?;

    println!("{}", &ntf[..(cols as usize)]);
    Ok(())
}

fn move_cursor(row: u16, col: u16) -> Result<()> {
    stdout().execute(cursor::MoveTo(col, row))?;
    Ok(())
}
