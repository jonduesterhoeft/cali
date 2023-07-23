//! A simple command line calendar implemented in Rust.
//! 
//! # Overview #
//! **cali** is a simple command line calendar. 
//! 
//! # Examples #
//!
use std::error::Error;
use std::path::PathBuf;
use clap::Parser;
use chrono::{DateTime, TimeZone, Timelike, NaiveDateTime, Utc, Local};
use chrono_tz::Tz;
use rusqlite::{params, Connection, Result};


pub struct Calendar {
    name: String,
    default: bool,
    offset: (i32, i32),
    path: PathBuf,
}

#[derive(Debug)]
struct CalendarExistsError;

impl std::fmt::Display for CalendarExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calendar with this name already exists.")
    }
}

impl std::error::Error for CalendarExistsError {}

impl Calendar {
    pub fn new(name: &str) -> Result<Calendar, Box<dyn Error>> {
        let path = PathBuf::from("calendar.db");
        check_existing_calendar(&path, &name)?;
        let existing_default = check_existing_default(&path)?;
        let mut set_default = false;
        if !existing_default {
            set_default = true;
        }

        let new_calendar = Calendar { 
            name: name.to_string(), 
            default: set_default, 
            offset: get_local_offset(), 
            path: path 
        };

        Ok(new_calendar)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_offset(&self) -> &(i32, i32) {
        &self.offset
    }

    pub fn get_default(&self) -> &bool {
        &self.default
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    // pub fn add_event(&mut self, event: Event<Tz>) -> Option<&Event<Tz>> {

    // }

    // pub fn next_event(&self) -> Option<&Event<Tz>> {

    // }

    // pub fn events_between(&self, start: NaiveDateTime, end: NaiveDateTime
    // ) -> impl Iterator<Item = &Event<Tz>> {

    // }
}


// /// A parser for command line input.
// /// 
// /// Reads the `query` and `path` arguments for the search along with a 
// /// number of options from the command line.
// /// 
// /// # Options #
// #[doc = include_str!("../examples/help.md")]
// ///
// #[derive(Parser)]
// #[command(author, version, about = "A simple to use command line calendar.", long_about = None)]
// pub struct InputParser {
//     #[arg(short, long)]
//     /// Event name
//     event: String,
//     #[arg(short, long)]
//     /// Ignores case whiles searching
//     new: bool,
// }

// pub struct Event<Tz: TimeZone> {
//     id: Uuid,
//     name: String,
//     start: DateTime<Tz>,
//     end: DateTime<Tz>,
//     recurring: bool,
// }

// impl<Tz: TimeZone> Event<Tz> {
//     pub fn new(
//         name: &str, 
//         start: Option<NaiveDateTime>,
//         end: Option<NaiveDateTime>,
//         recurring: Option<bool>
//     ) -> Event<Tz> {
//         Event { 
//             id: Uuid::new_v4(), 
//             name: name.to_string(), 
//             start: start.unwrap_or_else(|| NaiveDateTime::new()), 
//             end: end.unwrap_or_else(), 
//             recurring: recurring.unwrap_or_else(|| false),
//         }
//     }

//     fn get_id(&self) -> &Uuid {
//         &self.id
//     }

//     fn get_name(&self) -> &str {
//         &self.name
//     }

//     fn get_start(&self) -> &DateTime<Tz> {
//         &self.start
//     }

//     fn get_end(&self) -> &DateTime<Tz> {
//         &self.end
//     }

//     fn is_recurring(&self) -> bool {
//         self.recurring
//     }

//     fn update_name(&self, new_name: &str) -> Result<(), Box<dyn Error>> {
//         self.name = new_name.to_string();
//         Ok(())
//     }

//     fn update_start(&self, new_start: DateTime<Tz>) -> Result<(), Box<dyn Error>> {
//         self.start = new_start;
//         Ok(())
//     }

//     fn update_end(&self, new_end: DateTime<Tz>) -> Result<(), Box<dyn Error>> {
//         self.end = new_end;
//         Ok(())
//     }

// }

fn get_local_offset() -> (i32, i32) {
    let local_time = Local::now();
    let offset = local_time.offset();

    let offset_hours = offset.local_minus_utc() / 3600;
    let offset_minutes = (offset.local_minus_utc() % 3600) / 60;

    (offset_hours, offset_minutes)
}

fn new_database(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;

    // Create the calendar table if it doesn't already exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendars (
            event_id INTEGER PRIMARY KEY,
            calendar_name TEXT NOT NULL,
            event_name TEXT NOT NULL,
            event_start BLOB NOT NULL,
            event_end BLOB NOT NULL,
            event_recurring INTEGER NOT NULL,
            is_default INTEGER NOT NULL
        )",
        params![],
    )?;

    Ok(())
}

// Checks if there is already a calendar by the specified name
fn check_existing_calendar(path: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;
    let mut name_query = conn.prepare("SELECT calendar_name FROM calendars WHERE calendar_name = ?1")?;
    let mut name_check = name_query.query(params![name])?;

    if let Some(_row) = name_check.next()? {
        return Err(Box::new(CalendarExistsError));
    }

    Ok(())
}

// Check if there are any calendars set as default
fn check_existing_default(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let conn = Connection::open(path)?;
    let mut default_query = conn.prepare("SELECT is_default FROM calendars WHERE is_default <> 0")?;
    let mut default_check = default_query.query(params![])?;

    if let Some(_row) = default_check.next()? {
        Ok(true)
    } else {
        Ok(false)
    }
}

// Test helper function to insert a row with calendar_name of "test_calendar"
fn insert_test_calendar(path: &PathBuf, set_default: bool) -> Result<()> {
    let conn = Connection::open(path)?;
    // Insert a row with calendar_name "test_calendar"
    conn.execute(
        "INSERT INTO calendars (calendar_name, event_name, event_start, event_end, event_recurring, is_default) 
         VALUES (?1, 'Test Event', '2023-07-23', '2023-07-25', 0, ?2)",
        params!["test_calendar", set_default],
    )?;

    Ok(())
}

// Test helper function to remove the row with calendar_name of "test_calendar"
fn remove_test_calendar(path: &PathBuf) -> Result<()> {
    let conn = Connection::open(path)?;
    // Delete the row with calendar_name "test_calendar"
    conn.execute("DELETE FROM calendars WHERE calendar_name = ?1", params!["test_calendar"])?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_database() {
        let path = PathBuf::from("tests/test.db");
        let result = new_database(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_existing_calendar_does_exist() {
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path).unwrap();
        insert_test_calendar(&path, false).unwrap();
        let name = "test_calendar";
        let result = check_existing_calendar(&path, name);
        assert!(result.is_err());
        remove_test_calendar(&path).unwrap();
    }

    #[test]
    fn test_check_existing_calendar_does_not_exist() {
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path).unwrap();
        let name = "test_calendar";
        let result = check_existing_calendar(&path, name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_existing_default_does_exist() {
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path).unwrap();
        insert_test_calendar(&path, true).unwrap();
        let result = check_existing_default(&path).unwrap();
        assert!(result);
        remove_test_calendar(&path).unwrap();
    }

    #[test]
    fn test_check_existing_default_does_not_exist() {
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path).unwrap();
        insert_test_calendar(&path, false).unwrap();
        let result = check_existing_default(&path).unwrap();
        assert!(!result);
        remove_test_calendar(&path).unwrap();
    }

}