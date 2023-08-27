use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct GcndArgs {
    /// Action to perform
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Starts daemon if its not already running
    Start,
    /// Stops daemon if running
    Stop,
    /// Check if daemon is running or not
    Status,
}
