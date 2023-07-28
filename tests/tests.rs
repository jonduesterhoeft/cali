use cali::{calendar::*, event::*};
use std::path::PathBuf;
use rusqlite::{params, Connection, Result};

// Test Helper Methods

// Inserts a prebuilt test event under calendar 'name' into the database
fn insert_test_calendar(path: &PathBuf, name: &str, set_default: bool) -> Result<()> {
    let conn = Connection::open(path)?;
    // Insert a row with calendar_name "test_calendar"
    conn.execute(
        "INSERT INTO calendars (calendar_name, event_id, event_name, event_start, event_end, event_recurring, is_default) 
        VALUES (?1, '1', 'Test Event', '2023-07-23', '2023-07-25', 0, ?2)",
        params![name, set_default],
    )?;

    Ok(())
}

// Removes any events under calendar 'name' from the database
fn remove_test_calendar(path: &PathBuf, name: &str) -> Result<()> {
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

// Removes all entries in the database
fn remove_all_calendars(path: &PathBuf) -> Result<()> {
    let conn = Connection::open(path)?;

    // Check if the "calendars" table exists
    if let Ok(table_exists) = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'calendars'",
        params![],
        |_| Ok(true),
    ) {
        // If the table exists, execute the DELETE statement
        if table_exists {
            conn.execute("DELETE FROM calendars WHERE calendar_name IS NOT NULL", params![])?;
        }
    }

    Ok(())
}

// Cleans database and inserts a test calendar
fn clean_insert_test_calendar(path: &PathBuf, name: &str, set_default: bool) -> Result<()> {
    init_database(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    insert_test_calendar(&path, name, set_default).unwrap();
    Ok(())
}

// Cleans database and doesn't insert the test calendar
fn clean_noinsert_test_calendar(path: &PathBuf, name: &str) -> Result<()> {
    init_database(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    Ok(())
}

// Creates a test event with dummy info given 'name'
fn new_test_dummy_event(name: &str) -> Event {
    let start = "start_time";
    let end = "end_time";
    let recurring = Recurring::No;
    Event::new(name, start, end, recurring)
}



// Integration Tests


// Database

#[test]
fn test_new_database() {
    let path = PathBuf::from("tests/test.db");
    let result = init_database(&path);
    assert!(result.is_ok());
}

// Calendar

#[test]
fn test_verify_calendar_does_exist() {
    let name = "test calendar";
    let path = PathBuf::from("tests/test.db");
    let set_default = false;
    clean_insert_test_calendar(&path, &name, set_default).unwrap();
    let result = check_calendar(&path, name);
    assert!(result.unwrap());
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_delete_calendar_success() {
    let name = "test calendar";
    let path = PathBuf::from("tests/test.db");
    let set_default = false;
    clean_insert_test_calendar(&path, &name, set_default).unwrap();
    let calendar = Calendar::from(name, &path).unwrap();
    remove_calendar(&calendar).unwrap();
    let result = check_calendar(&path, name);
    assert!(!result.unwrap());
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_verify_calendar_does_not_exist() {
    let name = "test calendar";
    let path = PathBuf::from("tests/test.db");
    clean_noinsert_test_calendar(&path, name).unwrap();
    let result = check_calendar(&path, name);
    assert!(!result.unwrap());
}

#[test]
fn test_get_default_does_exist() {
    let path = PathBuf::from("tests/test.db");
    let default_calendar = "default calendar";
    let new_calendar = "test calendar";
    clean_insert_test_calendar(&path, &default_calendar, true).unwrap();
    clean_insert_test_calendar(&path, &new_calendar, false).unwrap();
    let result = get_default(&path).unwrap();
    assert_eq!(result, Some(default_calendar.to_string()));
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_get_default_does_not_exist() {
    let path = PathBuf::from("tests/test.db");
    let new_calendar = "test calendar";
    clean_insert_test_calendar(&path, &new_calendar, false).unwrap();
    let result = get_default(&path).unwrap();
    assert_eq!(result, None);
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_default_empty_database() {
    let path = PathBuf::from("tests/test.db");
    remove_all_calendars(&path).unwrap();
    let new_calendar = "test calendar";
    let result_check = check_default(&path, new_calendar).unwrap();
    assert_eq!(result_check, false);
    let result_get = get_default(&path).unwrap();
    assert_eq!(result_get, None);
}

#[test]
fn test_change_default() {
    let path = PathBuf::from("tests/test.db");
    let default_calendar = "default calendar";
    let new_calendar = "test calendar";
    clean_insert_test_calendar(&path, &default_calendar, true).unwrap();
    clean_insert_test_calendar(&path, &new_calendar, false).unwrap();
    let result = get_default(&path).unwrap();
    assert_eq!(result, Some(default_calendar.to_string()));
    assert_ne!(result, Some(new_calendar.to_string()));
    update_default(&path, new_calendar).unwrap();
    let result2 = get_default(&path).unwrap();
    assert_eq!(result2, Some(new_calendar.to_string()));
    assert_ne!(result2, Some(default_calendar.to_string()));
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_calendar_success_default_not_exists() {
    let name = "test calendar";
    let path = PathBuf::from("tests/test.db");
    clean_noinsert_test_calendar(&path, name).unwrap();
    let new_calendar = Calendar::new(&name, &path).unwrap();
    assert_eq!(new_calendar.get_name(), name);
    assert_eq!(new_calendar.get_path(), &path);
    assert_eq!(new_calendar.get_default(), &true);
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_calendar_success_default_exists() {
    let path = PathBuf::from("tests/test.db");
    let default_calendar = "default calendar";
    let name = "test calendar";
    clean_insert_test_calendar(&path, &default_calendar, true).unwrap();
    clean_noinsert_test_calendar(&path, &name).unwrap();
    let new_calendar = Calendar::new(name, &path).unwrap();
    assert_eq!(new_calendar.get_name(), name);
    assert_eq!(new_calendar.get_path(), &path);
    assert_eq!(new_calendar.get_default(), &false);
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_calendar_fail_name_exists() {
    let path = PathBuf::from("tests/test.db");
    let name = "test calendar";
    clean_insert_test_calendar(&path, &name, true).unwrap();
    let result = Calendar::new(name, &path);
    assert!(result.is_err());
    remove_all_calendars(&path).unwrap();
}

// Event

#[test]
fn test_new_event_success() {
    let name = "test_event";
    let event = new_test_dummy_event(name);
    assert_eq!(event.get_name(), name);
    // compare against known dummy values
    assert_eq!(event.get_start(), "start_time");
    assert_eq!(event.get_end(), "end_time");
    assert_eq!(event.get_recurring(), &Recurring::No);
}

#[test]
fn test_new_insert_get_exact_event_success() {
    // Create test calendar
    let path = PathBuf::from("tests/test.db");
    let calendar_name = "test calendar";
    clean_noinsert_test_calendar(&path, &calendar_name).unwrap();
    let calendar = Calendar::new(calendar_name, &path).unwrap();
    // Create test event
    let event_name = "test event";
    let event = new_test_dummy_event(event_name);
    // Insert
    insert_event(&calendar, &event).unwrap();
    // Get
    let exact = true;
    let got_event = get_event(&calendar, event_name, exact).unwrap();
    // Check values
    assert_eq!(got_event.len(), 1);
    assert_eq!(got_event[0].get_name(), event_name);
    // compare against known dummy values
    assert_eq!(got_event[0].get_start(), "start_time");
    assert_eq!(got_event[0].get_end(), "end_time");
    assert_eq!(got_event[0].get_recurring(), &Recurring::No);
}

#[test]
fn test_new_insert_get_nonexact_event_success() {
    // Create test calendar
    let path = PathBuf::from("tests/test.db");
    let calendar_name = "test calendar";
    clean_noinsert_test_calendar(&path, &calendar_name).unwrap();
    let calendar = Calendar::new(calendar_name, &path).unwrap();
    // Create test event
    let event_name = "test event";
    let event = new_test_dummy_event(event_name);
    // Insert
    insert_event(&calendar, &event).unwrap();
    // Get
    let exact = false;
    let search_name = "tes";
    let got_event = get_event(&calendar, search_name, exact).unwrap();
    // Check values
    assert_eq!(got_event.len(), 1);
    assert_eq!(got_event[0].get_name(), event_name);
    // compare against known dummy values
    assert_eq!(got_event[0].get_start(), "start_time");
    assert_eq!(got_event[0].get_end(), "end_time");
    assert_eq!(got_event[0].get_recurring(), &Recurring::No);
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_insert_get_multiple_nonexact_event_success() {
    // Create test calendar
    let path = PathBuf::from("tests/test.db");
    let calendar_name = "test calendar";
    clean_noinsert_test_calendar(&path, &calendar_name).unwrap();
    let calendar = Calendar::new(calendar_name, &path).unwrap();
    // Create and insert test events
    let event_name1 = "test event 1";
    let event1 = new_test_dummy_event(event_name1);
    insert_event(&calendar, &event1).unwrap();
    let event_name2 = "test event 2";
    let event2 = new_test_dummy_event(event_name2);
    insert_event(&calendar, &event2).unwrap();
    // Get
    let exact = false;
    let search_name = "test event";
    let got_event = get_event(&calendar, search_name, exact).unwrap();
    // Check values
    assert_eq!(got_event.len(), 2); 
    assert_eq!(got_event[0].get_name(), event_name1);
    assert_eq!(got_event[1].get_name(), event_name2);
    assert_ne!(got_event[0].get_name(), got_event[1].get_name());
    assert_ne!(got_event[0].get_id(), got_event[1].get_id());
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_insert_update_get_exact_event_success() {
    // Create test calendar
    let path = PathBuf::from("tests/test.db");
    let calendar_name = "test calendar";
    clean_noinsert_test_calendar(&path, &calendar_name).unwrap();
    let calendar = Calendar::new(calendar_name, &path).unwrap();
    // Create test event
    let event_name = "test event";
    let mut event = new_test_dummy_event(event_name);
    // Insert
    insert_event(&calendar, &event).unwrap();
    // Update
    let new_start = "new_start";
    event.update_start(new_start).unwrap();
    update_event(&calendar, &event).unwrap();
    // Get
    let exact = true;
    let got_event = get_event(&calendar, event_name, exact).unwrap();
    // Check values
    assert_eq!(got_event.len(), 1);
    assert_eq!(got_event[0].get_name(), event_name);
    // compare against known dummy values
    assert_eq!(got_event[0].get_start(), new_start);  // Verify update
    assert_eq!(got_event[0].get_end(), "end_time"); 
    assert_eq!(got_event[0].get_recurring(), &Recurring::No);
    remove_all_calendars(&path).unwrap();
}

#[test]
fn test_new_insert_get_multiple_nonexact_delete_event_success() {
    // Create test calendar
    let path = PathBuf::from("tests/test.db");
    let calendar_name = "test calendar";
    clean_noinsert_test_calendar(&path, &calendar_name).unwrap();
    let calendar = Calendar::new(calendar_name, &path).unwrap();
    // Create and insert test events
    let event_name1 = "test event 1";
    let event1 = new_test_dummy_event(event_name1);
    insert_event(&calendar, &event1).unwrap();
    let event_name2 = "test event 2";
    let event2 = new_test_dummy_event(event_name2);
    insert_event(&calendar, &event2).unwrap();
    // Delete event
    remove_event(&calendar, &event2).unwrap();
    // Get
    let exact = false;
    let search_name = "test event";
    let got_event = get_event(&calendar, search_name, exact).unwrap();
    // Check values
    assert_eq!(got_event.len(), 1); 
    assert_eq!(got_event[0].get_name(), event_name1);
    remove_all_calendars(&path).unwrap();
}