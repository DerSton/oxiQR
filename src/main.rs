use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "oxiqr",
    version,
    about = "Create QR codes from the command line",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a QR code from the given data
    Create {
        /// The data that should be encoded into the QR code
        data: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { data } => {
            println!("Would create QR code for: {data}");
        }
    }
}
