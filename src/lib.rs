#![feature(async_closure)]
pub mod app;
pub mod clipboard;
pub mod clipshare;
pub mod cfgfile;
pub mod connect;
pub mod consts;
pub mod ui;

// Use all the log macros automatically
#[macro_use] extern crate log;

use cfgfile::ConfigFile;
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
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Could not initialize project directories")
}

pub fn create_config(file: impl Into<String>) -> ConfigFile {
    let project_dirs = create_project_dirs();
    let config_dir = project_dirs.config_dir();
    ConfigFile::new(config_dir, file)
}
