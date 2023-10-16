use clap::{Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Episode(Episode),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Describe a Futurama episode (powered by Wikipedia)")]
pub struct Episode{
    name: Option<String>,
    #[arg(short, long, default_value_t=true)]
    all: bool,
}
