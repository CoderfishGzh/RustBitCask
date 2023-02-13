use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(author, version, about, long_about = None)]
#[warn(non_camel_case_types)]
enum Commands {
    set { key: String, value: String },
    get { key: String },
    rm { key: String },
}

pub fn parse_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::set { key, value }) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Some(Commands::get { key }) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Some(Commands::rm { key }) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        None => {
            exit(-1);
        }
    }
}
