use clap::{ Subcommand, Args};

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
        println!("Describe episode...{:?}", self.name);
    }
    
}
