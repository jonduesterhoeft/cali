use cali::*;
use cali::database::{remove_test_calendar, insert_test_calendar, database_setup};
use std::path::PathBuf;

#[test]
fn test_new_calendar_success() {
    let name = "test calendar";
    let path = PathBuf::from("calendar.db");
    database_setup(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    let new_calendar = Calendar::new(name).unwrap();
    assert_eq!(new_calendar.get_name(), "test calendar");
    assert_eq!(new_calendar.get_path(), &PathBuf::from("calendar.db"));
    assert_eq!(new_calendar.get_default(), &true);
    assert_eq!(new_calendar.get_offset(), &(-5, 0));  // testing in CDT
}

#[test]
fn test_new_calendar_success_default_exists() {
    let name = "test calendar";
    let path = PathBuf::from("calendar.db");
    database_setup(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    insert_test_calendar(&path, "default calendar", true).unwrap();
    let new_calendar = Calendar::new(name).unwrap();
    assert_eq!(new_calendar.get_name(), "test calendar");
    assert_eq!(new_calendar.get_path(), &PathBuf::from("calendar.db"));
    assert_eq!(new_calendar.get_default(), &false);
    assert_eq!(new_calendar.get_offset(), &(-5, 0));  // testing in CDT
    remove_test_calendar(&path, "default calendar").unwrap();
}

#[test]
fn test_new_calendar_fail_name_exists() {
    let name = "test calendar";
    let path = PathBuf::from("calendar.db");
    database_setup(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    insert_test_calendar(&path, name, false).unwrap();
    let failure = match Calendar::new(name) {
        Ok(_) => false,
        Err(_) => true
    };
    assert!(failure);
}