use crate::{event::*, cali_error::*};
use std::path::PathBuf;
use std::error::Error;
use rusqlite::{params, Connection, Result};


pub struct Calendar {
    name: String,
    default: bool,
    path: PathBuf,
}

impl Calendar {
    
    pub fn new(name: &str, path: &PathBuf) -> Result<Calendar, Box<dyn Error>> {
        init_database(&path).unwrap();
        if check_calendar(&path, &name)? {
            return Err(Box::new(CalendarExistsError));
        }
        let existing_default = get_default(&path)?;

        Ok(Calendar { 
            name: name.to_string(), 
            default: existing_default.is_none(), 
            path: path.to_path_buf()
        })
    }

    pub fn from(name: &str, path: &PathBuf) -> Result<Calendar, Box<dyn Error>> {
        init_database(&path).unwrap();

        if check_calendar(&path, name)? {
            Calendar::from_existing(name, path)
        } else {
            Calendar::new(name, path)
        }
    }

    fn from_existing(name: &str, path: &PathBuf) -> Result<Calendar, Box<dyn Error>> {
        Ok( Calendar { 
            name: name.to_string(), 
            default: check_default(&path, name).unwrap(),
            path: path.to_path_buf()
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_default(&self) -> &bool {
        &self.default
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn add_event(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        insert_event(&self, &event)?;
        Ok(())
    }

    pub fn update_event(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        update_event(&self, &event)?;
        Ok(())
    }

    pub fn remove_event(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        remove_event(&self, &event)?;
        Ok(())
    }

    // pub fn next_event(&self) -> Option<&Event<Tz>> {

    // }

    // pub fn events_between(&self, start: NaiveDateTime, end: NaiveDateTime
    // ) -> impl Iterator<Item = &Event<Tz>> {

    // }
}


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