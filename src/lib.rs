//! A simple command line calendar implemented in Rust.
//! 
//! # Overview #
//! **cali** is a simple command line calendar. 
//! 
//! # Examples #
//!
pub mod calendar;
pub mod event;
pub mod time;
pub mod database;
pub mod cali_error;
use clap::Parser;



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

