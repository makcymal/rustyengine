use {
    anyhow::Result,
    crossterm::{
        event, cursor,
        ExecutableCommand,
        terminal::{enable_raw_mode, size},
    },
    super::activity::*,
    std::{
        thread,
        time::Duration,
        io::stdout,
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

pub fn listen() -> Result<Activity> {
    let activity = event::read()?;
    Ok(Activity::from(activity))
}

pub fn show_message(msg: &str, rows: u16, cols: u16) -> Result<()> {
    let col = cols.saturating_sub(msg.len() as u16) / 2;
    let row = rows / 2;
    move_cursor(row, col)?;

    clear();
    println!("{}",  msg[..cols]);
    thread::sleep(Duration::from_millis(timeout));
    clear();
    Ok(())
}

pub fn show_notification(ntf: &str) -> Result<()> {
    let col = cols.saturating_sub(msg.len() as u16) / 2;
    move_cursor(row, col)?;

    println!("{}",  ntf[..cols]);
    Ok(())
}

fn move_cursor(row: u16, col: u16) -> Result<()> {
    stdout().execute(cursor::MoveTo(col, row))?;
    Ok(())
}
