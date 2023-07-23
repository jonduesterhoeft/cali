use cali::*;
use cali::calendar::*;
use cali::database::{remove_test_calendar, insert_test_calendar, init_database, update_default};
use std::path::PathBuf;

#[test]
fn test_new_calendar_success() {
    let name = "test calendar";
    let path = PathBuf::from("calendar.db");
    init_database(&path).unwrap();
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
    init_database(&path).unwrap();
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
    init_database(&path).unwrap();
    remove_test_calendar(&path, name).unwrap();
    insert_test_calendar(&path, name, false).unwrap();
    let failure = match Calendar::new(name) {
        Ok(_) => false,
        Err(_) => true
    };
    assert!(failure);
}

// #[test]
// fn test_change_default() {
//     let path = PathBuf::from("calendar.db");
//     init_database(&path).unwrap();
//     // Current default
//     let name = "test calendar";
//     remove_test_calendar(&path, name).unwrap();
//     insert_test_calendar(&path, name, true).unwrap();
//     // Want to change this to be the new default
//     let new_default = "new default calendar";
//     remove_test_calendar(&path, new_default).unwrap();
//     insert_test_calendar(&path,new_default, false).unwrap();
//     update_default(path, new_default).unwrap();
    
//     remove_test_calendar(&path, name).unwrap();
//     remove_test_calendar(&path, new_default).unwrap();
// }