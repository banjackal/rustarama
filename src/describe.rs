use clap::{ Subcommand, Args};

#[path = "./infosphere.rs"]
mod infosphere;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command()]
    Episode(Episode),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Describe a Futurama episode (powered by Infosphere)")]
pub struct Episode {
    #[arg(long)]
    name: String,
}

impl Episode {
    pub fn describe_episode(&self) {
        infosphere::describe_episode(&self.name).unwrap()
    }
}
