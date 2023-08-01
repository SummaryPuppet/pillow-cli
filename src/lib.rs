mod init;
mod make;
mod run;

use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new project with cargo-generate
    Init,

    /// Run the server with cargo-watch
    Run,

    /// Make
    Make(make::Make),
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Some(command) => match command {
                Commands::Init => {
                    init::init();
                }

                Commands::Run => {
                    run::run();
                }

                Commands::Make(s) => match &s.command {
                    make::Commands::Route(r) => {
                        println!("route: {:#?}", r);
                    }
                    make::Commands::Migrate => println!("migrate"),
                },
            },
            None => {
                exit(0);
            }
        }
    }
}
