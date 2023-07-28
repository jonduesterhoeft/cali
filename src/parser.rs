use crate::{calendar::*};
use clap::Parser;
use std::io;
use std::error::Error;
use std::path::PathBuf;

/// A parser for command line input.
/// 
/// Reads the `query` and `path` arguments for the search along with a 
/// number of options from the command line.
/// 
/// # Options #
#[doc = include_str!("../examples/help.md")]
///
#[derive(Parser)]
#[command(author, version, about = "A simple to use command line calendar.", long_about = None)]
pub struct InputParser {
    /// Calendar name. Selects the specified calendar to use. 
    /// If there is not a current calendar using that name then a new one is created. 
    /// If no name is given then the default calendar is used. 
    /// If no default exists, then a new calendar named "default calendar" is created.
    calendar_name: Option<String>,
    #[arg(short, long)]
    /// Deletes the specified calendar
    delete: bool,
    #[arg(short, long)]
    /// Renames the specified calendar
    rename: bool,
    #[arg(short, long)]
    /// Sets the specified calendar as default
    set_default: bool,
}

/// Defines methods expected to run on `InputParser`.
pub trait RunArgs {
    /// Executes the search process given the command line arguments.
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl InputParser {
    /// Creates a new `InputParser`.
    /// 
    /// # Returns
    /// Returns a `InputParser` containing the specified arguments.
    /// 
    /// # Example
    /// ```
    /// # use crate::cali::InputParser;
    /// # use std::path::PathBuf;
    /// let calendar_name = "Jon's Calendar";
    /// let delete = false;
    /// let rename = false;
    /// let set_default = true;
    /// 
    /// let new_parser = InputParser { calendar_name,, delete, rename, set_default };
    /// ```
    /// 
    pub fn new(calendar_name: Option<String>, delete: bool, rename: bool, set_default: bool) -> InputParser {
        InputParser {
            calendar_name, 
            delete,
            rename,
            set_default
        }
    }
}

impl RunArgs for InputParser {
    /// 
    /// # Returns
    /// Returns () if successful.
    /// 
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("calendar.db");
        init_database(&path)?;

        let name: String;
        if let None = self.calendar_name {
            if let Some(default_name) = get_default(&path)? {
                name = default_name;
            } else {
                name = "default calendar".to_string();
            }
        } else {
            name = self.calendar_name.as_ref().unwrap().to_string();
        }

        let mut calendar = Calendar::from(&name, &path)?;

        if self.delete {
            remove_calendar(&calendar)?;
            println!("'{}' was deleted.", calendar.get_name());
            return Ok(());
        }

        if self.rename {
            let mut new_name = String::new();
            println!("Enter a new name for calendar: '{}'", calendar.get_name());
            io::stdin().read_line(&mut new_name).expect("failed to readline");
            new_name = new_name.trim().to_string();
            calendar.update_name(&new_name)?;
            println!("'{}' was renamed to '{}'.", name, new_name);
        }

        if self.set_default {
            update_default(&path, calendar.get_name())?;
            println!("'{}' is now set as default.", calendar.get_name());
        }

        Ok(())
    }
}