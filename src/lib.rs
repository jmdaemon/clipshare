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
