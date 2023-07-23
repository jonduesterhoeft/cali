//! A simple command line calendar implemented in Rust.
//! 
//! # Overview #
//! **cali** is a simple command line calendar. 
//! 
//! # Examples #
//!
pub mod calendar;
pub mod time;
pub mod database;
pub mod cali_error;
use crate::database::*;
use crate::time::*;
use std::error::Error;
use std::path::PathBuf;
use clap::Parser;
use chrono::{DateTime, TimeZone, Timelike, NaiveDateTime, Utc, Local};
use uuid::Uuid;


pub struct Calendar {
    name: String,
    default: bool,
    offset: (i32, i32),
    path: PathBuf,
}



impl Calendar {
    pub fn new(name: &str) -> Result<Calendar, Box<dyn Error>> {
        let path = PathBuf::from("calendar.db");
        database_setup(&path).unwrap();
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

pub enum Recurring {
    No,
    Daily,
    Weekly,
    Monthly,
    Yearly
}


pub struct Event {
    id: Uuid,
    name: String,
    start: String,
    end: String,
    offset: (i32, i32),
    recurring: Recurring,
}

// impl Event {
//     pub fn new(name: &str, start: &str, end: &str, recurring: Recurring) -> Event {
//         Event { 
//             id: Uuid::new_v4(), 
//             name: name.to_string(), 
//             start: start.unwrap_or_else(|| NaiveDateTime::new()), 
//             end: end.unwrap_or_else(), 
//             offset: get_local_offset(),
//             recurring: Recurring::No,
//         }
//     }

//     fn get_id(&self) -> &Uuid {
//         &self.id
//     }

//     fn get_name(&self) -> &str {
//         &self.name
//     }

//     fn get_start(&self) -> &str{
//         &self.start
//     }

//     fn get_end(&self) -> &str {
//         &self.end
//     }

//     fn is_recurring(&self) -> &Recurring {
//         &self.recurring
//     }

//     fn update_name(&self, new_name: &str) -> Result<(), Box<dyn Error>> {
//         self.name = new_name.to_string();
//         Ok(())
//     }

//     fn update_start(&self, new_start: &str) -> Result<(), Box<dyn Error>> {
//         self.start = new_start.to_string();
//         Ok(())
//     }

//     fn update_end(&self, new_end: &str) -> Result<(), Box<dyn Error>> {
//         self.end = new_end.to_string();
//         Ok(())
//     }

// }



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

