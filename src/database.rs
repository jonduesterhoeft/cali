use std::path::PathBuf;
use std::error::Error;
use rusqlite::{params, Connection, Result};
use crate::cali_error::CalendarExistsError;

pub fn database_setup(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;

    // Create the calendar table if it doesn't already exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendars (
            event_id TEXT,
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
pub fn check_existing_calendar(path: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(path)?;
    let mut name_query = conn.prepare("SELECT calendar_name FROM calendars WHERE calendar_name = ?1")?;
    let mut name_check = name_query.query(params![name])?;

    if let Some(_row) = name_check.next()? {
        return Err(Box::new(CalendarExistsError));
    }

    Ok(())
}

// Check if there are any calendars set as default
pub fn check_existing_default(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
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
pub fn insert_test_calendar(path: &PathBuf, name: &str, set_default: bool) -> Result<()> {
    let conn = Connection::open(path)?;
    // Insert a row with calendar_name "test_calendar"
    conn.execute(
        "INSERT INTO calendars (calendar_name, event_name, event_start, event_end, event_recurring, is_default) 
        VALUES (?1, 'Test Event', '2023-07-23', '2023-07-25', 0, ?2)",
        params![name, set_default],
    )?;

    Ok(())
}

// Test helper function to remove the row with calendar_name of "test_calendar"
pub fn remove_test_calendar(path: &PathBuf, name: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    // Check if the "calendars" table exists
    if let Ok(table_exists) = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'calendars'",
        params![],
        |_| Ok(true),
    ) {
        // If the table exists, execute the DELETE statement
        if table_exists {
            conn.execute("DELETE FROM calendars WHERE calendar_name = ?1", params![name])?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_database() {
        let path = PathBuf::from("tests/test.db");
        let result = database_setup(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_existing_calendar_does_exist() {
        let name = "test calendar";
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path, name).unwrap();
        insert_test_calendar(&path, name, false).unwrap();
        let result = check_existing_calendar(&path, name);
        assert!(result.is_err());
        remove_test_calendar(&path, name).unwrap();
    }

    #[test]
    fn test_check_existing_calendar_does_not_exist() {
        let name = "test calendar";
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path, name).unwrap();
        let name = "test_calendar";
        let result = check_existing_calendar(&path, name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_existing_default_does_exist() {
        let name = "test calendar";
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path, name).unwrap();
        insert_test_calendar(&path, name, true).unwrap();
        let result = check_existing_default(&path).unwrap();
        assert!(result);
        remove_test_calendar(&path, name).unwrap();
    }

    #[test]
    fn test_check_existing_default_does_not_exist() {
        let name = "test calendar";
        let path = PathBuf::from("tests/test.db");
        remove_test_calendar(&path, name).unwrap();
        insert_test_calendar(&path, name, false).unwrap();
        let result = check_existing_default(&path).unwrap();
        assert!(!result);
        remove_test_calendar(&path, name).unwrap();
    }

}