#![allow(clippy::all)]
// I fucking hate you clippy, so fucking aggresive towards the programmer.

// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
// This needs to live in main.rs rather than windows.rs because it needs to be a crate-level
// attribute, and it doesn't affect the mac build at all, so it's innocuous to leave for
// both target_os.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod config;
mod utils;
mod windows;

use crate::windows as os;

use anyhow::Result;

fn main() -> Result<()> {
    os::main()?;

    // Keeps the console open to check for crashes.
    // let _ = std::process::Command::new("cmd.exe").arg("/c").arg("pause").status();
    Ok(())
}
