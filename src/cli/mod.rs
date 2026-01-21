//! CLI module - argument parsing and command execution

pub mod args;
pub mod commands;

pub use args::{Cli, Commands, GenerateArgs, ServeArgs, ValidateArgs};
