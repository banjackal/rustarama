use clap::{Parser, Subcommand, CommandFactory };

mod describe;
mod get;
mod infosphere;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    #[command(long_about="Generate the autocompletion script for the specified shell")]
    Completion {
        #[arg(value_enum, help="Generate the autocompletion script for the specified shell")]
        shell: clap_complete_command::Shell,
    },
    #[command(long_about="Describe a Futurama episode (powered by Infosphere)")]
    Describe {
        #[command(subcommand)]
        command: describe::Commands
    },
    #[command(long_about="Get quote or list of episodes")]
    Get {
        #[command(subcommand)]
        command: get::Commands
    },
}

fn handle_episodes(episode: &get::Episodes) {
    if let Some(season) = episode.season {
        infosphere::get_episodes(Some(season)).unwrap()
    }
    if episode.all {
        infosphere::get_episodes(None).unwrap()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Completion { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
        Commands::Get { command } => {
            match command {
                get::Commands::Characters(_) => {
                   get::Characters::print() 
                }
                get::Commands::Episodes(e) => {
                    handle_episodes(e)
                }
                get::Commands::Quote(_) => {
                    println!("Getting quote...")
                }
            }
        }
        Commands::Describe { command } => {
            match command {
                describe::Commands::Episode { name } => {
                    println!("Describe episode...{:?}", name);
                }
            }
        }
    }

    Ok(())
}
