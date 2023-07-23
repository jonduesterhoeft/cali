use crate::database::*;
use crate::time::*;
use std::error::Error;
use uuid::Uuid;
use chrono::{DateTime, TimeZone, Timelike, NaiveDateTime, Utc, Local};

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

impl Event {
    pub fn new(name: &str, start: &str, end: &str, recurring: Recurring) -> Event {
        Event { 
            id: Uuid::new_v4(), 
            name: name.to_string(), 
            start: start.to_string(), 
            end: end.to_string(), 
            offset: get_local_offset(),
            recurring: Recurring::No,
        }
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_start(&self) -> &str{
        &self.start
    }

    fn get_end(&self) -> &str {
        &self.end
    }

    fn is_recurring(&self) -> &Recurring {
        &self.recurring
    }

    fn update_name(&mut self, new_name: &str) -> Result<(), Box<dyn Error>> {
        self.name = new_name.to_string();
        Ok(())
    }

    fn update_start(&mut self, new_start: &str) -> Result<(), Box<dyn Error>> {
        self.start = new_start.to_string();
        Ok(())
    }

    fn update_end(&mut self, new_end: &str) -> Result<(), Box<dyn Error>> {
        self.end = new_end.to_string();
        Ok(())
    }

}
