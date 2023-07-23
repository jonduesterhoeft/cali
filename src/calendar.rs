use std::path::PathBuf;
use std::error::Error;
use crate::database::*;
use crate::time::*;

pub struct Calendar {
    name: String,
    default: bool,
    offset: (i32, i32),
    path: PathBuf,
}

impl Calendar {
    pub fn new(name: &str) -> Result<Calendar, Box<dyn Error>> {
        let path = PathBuf::from("calendar.db");
        init_database(&path).unwrap();
        check_existing_calendar(&path, &name)?;
        let existing_default = check_existing_default(&path)?;

        let new_calendar = Calendar { 
            name: name.to_string(), 
            default: !existing_default, 
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