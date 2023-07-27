use crate::{database::*, event::*, cali_error::*};

use std::path::PathBuf;
use std::error::Error;


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