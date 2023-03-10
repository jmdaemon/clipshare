pub mod app;
pub mod config;
pub mod consts;
pub mod device;
pub mod discover;
pub mod ws;
// User Interfaces
#[cfg(target_os = "linux")]
#[cfg(feature = "gtk")]
pub mod gtk;
#[cfg(target_os = "windows")]
pub mod winui;

// Use all the log macros automatically
#[macro_use] extern crate log;

// Generic library helper functions
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
