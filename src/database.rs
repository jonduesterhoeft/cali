
use crate::{calendar::*, event::*};
use std::path::PathBuf;
use std::error::Error;
use rusqlite::{params, Connection, Result};


// Create the calendar table if it doesn't already exist
pub fn init_database(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendars (
            event_id TEXT NOT NULL,
            calendar_name TEXT NOT NULL,
            event_name TEXT NOT NULL,
            event_start TEXT NOT NULL,
            event_end TEXT NOT NULL,
            event_recurring TEXT NOT NULL,
            is_default INTEGER NOT NULL
        )",
        params![],
    )?;

    Ok(())
}

// Checks if there is a calendar by the specified name
pub fn check_calendar(path: &PathBuf, name: &str) -> Result<bool, Box<dyn Error>> {
    let conn = Connection::open(path)?;
    let check_name: Result<String> = conn.query_row(
        "SELECT calendar_name FROM calendars WHERE calendar_name = ?1",
        params![name],
        |row| row.get(0),
    );

    match check_name {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => Err(Box::new(e)),
    }
}

// Checks if there is an existing calendar set to default
pub fn check_default(path: &PathBuf, name: &str) -> Result<bool, Box<dyn Error>> {
    let conn = Connection::open(path)?;
    let check_name: Result<String> = conn.query_row(
        "SELECT is_default FROM calendars WHERE calendar_name = ?1 AND is_default = 1",
        params![name],
        |row| row.get(0),
    );

    match check_name {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => Err(Box::new(e)),
    }
}

// Gets the name of the default calendar
pub fn get_default(path: &PathBuf) -> Result<Option<String>, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    let default_calendar_name: Result<Option<String>> = conn.query_row(
        "SELECT calendar_name FROM calendars WHERE is_default = 1",
        params![],
        |row| row.get(0),
    );

    match default_calendar_name {
        Ok(name) => Ok(name),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(Box::new(e)),
    }
}

// Udpates the specified calendar to be the default
pub fn update_default(path: &PathBuf, new_default: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;
    // Reset calendar currently set to be the default
    let mut remove_current = conn.prepare("UPDATE calendars SET is_default = 0 WHERE is_default <> 0")?;
    remove_current.execute(params![])?;
    // Set the specified calendar as the new default
    let mut update_default = conn.prepare("UPDATE calendars SET is_default = 1 WHERE calendar_name = ?1")?;
    update_default.execute(params![new_default])?;

    Ok(())
}

// Inserts a new event into the database
pub fn insert_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "INSERT INTO calendars (calendar_name, event_id, event_name, event_start, event_end, event_recurring, is_default) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            calendar.get_name().to_string(), 
            event.get_id().to_string(), 
            event.get_name().to_string(), 
            event.get_start().to_string(), 
            event.get_end().to_string(), 
            event.get_recurring().to_string(), 
            calendar.get_default()
            ],
    )?;

    Ok(())
}

// Reads an existing event from the database
pub fn get_event(calendar: &Calendar, name: &str, exact: bool) -> Result<Vec<Event>, Box<dyn Error>> {
    let conn = Connection::open(calendar.get_path())?;

    let get_query = if exact {
        "SELECT * FROM calendars WHERE calendar_name = ?1 AND event_name = ?2"
    } else {
        "SELECT * FROM calendars WHERE calendar_name = ?1 AND event_name LIKE ?2"
    };

    let event_name = if exact {
        name.to_string()
    } else {
        format!("%{}%", name)
    };

    let mut stmt = conn.prepare(get_query)?;
    let event_iter = stmt.query_map(params![calendar.get_name(), event_name], |row| {
        let id: String = row.get("event_id")?;
        let name: String = row.get("event_name")?;
        let start: String = row.get("event_start")?;
        let end: String = row.get("event_end")?;
        let recurring_str: String = row.get("event_recurring")?;

        // Parse the recurring field from the database string representation into the Recurring enum
        let recurring = match recurring_str.as_str() {
            "No" => Recurring::No,
            "Daily" => Recurring::Daily,
            "Weekly" => Recurring::Weekly,
            "Monthly" => Recurring::Monthly,
            "Yearly" => Recurring::Yearly,
            _ => Recurring::No, // Handle unknown values, you may want to adjust this based on your data
        };

        Ok(Event::from(&id, &name, &start, &end, recurring))
    })?;

    let mut events = Vec::new();
    for event_result in event_iter {
        events.push(event_result?);
    }

    Ok(events)
}

// Updates an existing event in the database
pub fn update_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "UPDATE calendars
            SET calendar_name = ?1, 
            event_id = ?2, 
            event_name = ?3, 
            event_start = ?4, 
            event_end = ?5, 
            event_recurring = ?6, 
            is_default = ?7 
        WHERE event_id = ?2",
        params![
            calendar.get_name().to_string(), 
            event.get_id().to_string(), 
            event.get_name().to_string(), 
            event.get_start().to_string(), 
            event.get_end().to_string(), 
            event.get_recurring().to_string(), 
            calendar.get_default()
            ],
    )?;

    Ok(())
}

// Removes an existing event from the database
pub fn remove_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "DELETE FROM calendars WHERE calendar_name = ?1 AND event_id = ?2",
        params![calendar.get_name().to_string(), event.get_id().to_string()],
    )?;

    Ok(())
}