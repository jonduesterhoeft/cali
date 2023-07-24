use crate::{database::*, time::*};
use std::fmt;
use std::error::Error;
use uuid::Uuid;

pub enum Recurring {
    No,
    Daily,
    Weekly,
    Monthly,
    Yearly
}

impl fmt::Display for Recurring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Recurring::No => write!(f, "No"),
            Recurring::Daily => write!(f, "Daily"),
            Recurring::Weekly => write!(f, "Weekly"),
            Recurring::Monthly => write!(f, "Monthly"),
            Recurring::Yearly => write!(f, "Yearly"),
        }
    }
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
            recurring,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_start(&self) -> &str{
        &self.start
    }

    pub fn get_end(&self) -> &str {
        &self.end
    }

    pub fn get_offset(&self) -> &(i32, i32) {
        &self.offset
    }

    pub fn get_recurring(&self) -> &Recurring {
        &self.recurring
    }

    pub fn update_name(&mut self, new_name: &str) -> Result<(), Box<dyn Error>> {
        self.name = new_name.to_string();
        Ok(())
    }

    pub fn update_start(&mut self, new_start: &str) -> Result<(), Box<dyn Error>> {
        self.start = new_start.to_string();
        Ok(())
    }

    pub fn update_end(&mut self, new_end: &str) -> Result<(), Box<dyn Error>> {
        self.end = new_end.to_string();
        Ok(())
    }

}
