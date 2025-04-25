use clap::{Command, Parser, Subcommand};
pub mod args;

// Add more module declarations here as needed
fn main() {
    let args = Args::parse();
    match args.command {
        Commands::ServerVersion => {
            println!("Server version: {:?}", args.endpoint);
        }
        Commands::HealthCheck => {
            println!("Health check: {:?}", args.endpoint);
        }
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    endpoint: Option<String>,

    #[clap(short, long, default_value = "processed")]
    commitment: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ServerVersion,
    HealthCheck,
}
