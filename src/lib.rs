mod init;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(init::Options),
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Init(options)) => {
                options.init_project();
            }
            None => {}
        }
    }
}
