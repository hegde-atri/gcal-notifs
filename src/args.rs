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
    #[clap(name = "start", about = "Starts the daemon")]
    Start(StartArgs),
    /// Stops daemon if running
    Stop,
    /// Check if daemon is running or not
    Status,
    /// Coming soon... (Mute notification sound)
    Mute,
    /// Coming soon... (Unmute notification sound)
    Unmute,
}

#[derive(Debug, Parser)]
pub struct StartArgs {
    /// Mute the sound
    #[clap(long)]
    pub muted: bool,
}
