use clap::{Parser, Subcommand, CommandFactory };

mod describe;
mod get;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {
    Completion {
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
    Describe {
        #[command(subcommand)]
        command: Option<describe::Commands>
    },
    Get {
        #[command(subcommand)]
        command: Option<get::Commands>
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    match &cli.command.unwrap() {
        Commands::Completion { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
        Commands::Get { command } => {
            println!("Get episodes...{:?}", command);

        }
        Commands::Describe { command } => {
            println!("Describe episodes...{:?}", command);
        }
    }

    Ok(())
}
