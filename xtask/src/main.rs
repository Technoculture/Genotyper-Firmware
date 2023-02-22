use clap::{Parser, Subcommand};

/// Commands
#[derive(Subcommand, Debug)]
enum Command {
    /// Serial Devices
    Serial {
        /// List all serial devices
        #[arg(short, long)]
        list: bool,
    },
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Command,

    /// The verbosity level
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    if args.verbose {
        println!("Hello {:?}!", args.command);
    }

    match args.command {
        Command::Serial { list } => {
            if list {
                println!("Listing serial devices");
            } else {
                println!("No command given");
            }
        }
    }
}
