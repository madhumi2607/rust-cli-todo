use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Rust Todo")]
#[command(about = "Simple CLI todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task } => {
            println!("Task added: {}", task);
        }
        Commands::List => {
            println!("Listing tasks...");
        }
    }
}