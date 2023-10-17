use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(author, version, about, long_about = "Describe a Futurama episode (powered by Infosphere)")]
    Episode {
        #[arg(long)]
        name: String,
    },
}
