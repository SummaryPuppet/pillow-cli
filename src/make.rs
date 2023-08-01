use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Make {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new route
    Route(Route),

    /// Create migrations
    Migrate,
}

#[derive(Args, Debug)]
pub struct Route {
    name: String,
}
