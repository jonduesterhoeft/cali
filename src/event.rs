use crate::calendar::*;
use std::fmt;
use std::error::Error;
use uuid::Uuid;
use rusqlite::{params, Connection, Result};

#[derive(Debug, PartialEq)]
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
    recurring: Recurring,
}

impl Event {
    pub fn new(name: &str, start: &str, end: &str, recurring: Recurring) -> Event {
        Event { 
            id: Uuid::new_v4(), 
            name: name.to_string(), 
            start: start.to_string(), 
            end: end.to_string(), 
            recurring,
        }
    }

    pub fn from(id: &str, name: &str, start: &str, end: &str, recurring: Recurring) -> Event {
        Event { 
            id: Uuid::parse_str(id).unwrap(),
            name: name.to_string(), 
            start: start.to_string(), 
            end: end.to_string(), 
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


// Inserts a new event into the database
pub fn insert_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "INSERT INTO calendars (calendar_name, event_id, event_name, event_start, event_end, event_recurring, is_default) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            calendar.get_name().to_string(), 
            event.get_id().to_string(), 
            event.get_name().to_string(), 
            event.get_start().to_string(), 
            event.get_end().to_string(), 
            event.get_recurring().to_string(), 
            calendar.get_default()
            ],
    )?;

    Ok(())
}

// Reads an existing event from the database
pub fn get_event(calendar: &Calendar, name: &str, exact: bool) -> Result<Vec<Event>, Box<dyn Error>> {
    let conn = Connection::open(calendar.get_path())?;

    let get_query = if exact {
        "SELECT * FROM calendars WHERE calendar_name = ?1 AND event_name = ?2"
    } else {
        "SELECT * FROM calendars WHERE calendar_name = ?1 AND event_name LIKE ?2"
    };

    let event_name = if exact {
        name.to_string()
    } else {
        format!("%{}%", name)
    };

    let mut stmt = conn.prepare(get_query)?;
    let event_iter = stmt.query_map(params![calendar.get_name(), event_name], |row| {
        let id: String = row.get("event_id")?;
        let name: String = row.get("event_name")?;
        let start: String = row.get("event_start")?;
        let end: String = row.get("event_end")?;
        let recurring_str: String = row.get("event_recurring")?;

        // Parse the recurring field from the database string representation into the Recurring enum
        let recurring = match recurring_str.as_str() {
            "No" => Recurring::No,
            "Daily" => Recurring::Daily,
            "Weekly" => Recurring::Weekly,
            "Monthly" => Recurring::Monthly,
            "Yearly" => Recurring::Yearly,
            _ => Recurring::No, // Handle unknown values, you may want to adjust this based on your data
        };

        Ok(Event::from(&id, &name, &start, &end, recurring))
    })?;

    let mut events = Vec::new();
    for event_result in event_iter {
        events.push(event_result?);
    }

    Ok(events)
}

// Updates an existing event in the database
pub fn update_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "UPDATE calendars
            SET calendar_name = ?1, 
            event_id = ?2, 
            event_name = ?3, 
            event_start = ?4, 
            event_end = ?5, 
            event_recurring = ?6, 
            is_default = ?7 
        WHERE event_id = ?2",
        params![
            calendar.get_name().to_string(), 
            event.get_id().to_string(), 
            event.get_name().to_string(), 
            event.get_start().to_string(), 
            event.get_end().to_string(), 
            event.get_recurring().to_string(), 
            calendar.get_default()
            ],
    )?;

    Ok(())
}

// Removes an existing event from the database
pub fn remove_event(calendar: &Calendar, event: &Event) -> Result<()> {
    let conn = Connection::open(calendar.get_path())?;
    conn.execute(
        "DELETE FROM calendars WHERE calendar_name = ?1 AND event_id = ?2",
        params![calendar.get_name().to_string(), event.get_id().to_string()],
    )?;

    Ok(())
}