pub mod app;
pub mod clipshare;
pub mod config;
pub mod connect;
pub mod consts;
// User Interfaces
#[cfg(target_os = "linux")]
#[cfg(feature = "gtk")]
pub mod gtk;
#[cfg(target_os = "windows")]
pub mod winui;

// Use all the log macros automatically
#[macro_use] extern crate log;

// Generic library helper functions
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
