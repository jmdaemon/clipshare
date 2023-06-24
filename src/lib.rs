#![feature(async_closure)]
pub mod app;
pub mod clipshare;
pub mod config;
pub mod connect;
pub mod consts;
pub mod ui;

// Use all the log macros automatically
#[macro_use] extern crate log;

use config::Config;
use consts::{QUALIFIER, ORGANIZATION, APPLICATION};
use directories::ProjectDirs;

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

pub fn create_project_dirs() -> ProjectDirs {
    let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Could not initialize project directories");
    project_dirs
}

pub fn create_config(file: impl Into<String>) -> Config {
    let project_dirs = create_project_dirs();
    let config_dir = project_dirs.config_dir();
    Config::new(config_dir, file)
}
