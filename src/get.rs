use clap::{ Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command()]
    Episodes(Episodes),
    #[command()]
    Characters,
    Quote(Quote),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Get list of episodes from series or season")]
pub struct Episodes {
    name: Option<String>,
    #[arg(short, long, help="Season number (1-7)", value_name="int")]
    season: Option<i32>,
    #[arg(short, long, help="Show episodes from all seasons")]
    all: bool,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Get random Futurama quote")]
pub struct Quote {
    #[arg(short, long, help="Character name (e.g. 'Fry', 'Bender'")]
    character: Option<String>,
    #[arg(short, long, help="Episode name (use 'futurama get episodes' command for assistance)")]
    episode: Option<String>,
    #[arg(short, long, help="Season number (1-7)", value_name="int")]
    season: Option<i32>,
    #[arg(short, long, help="Show episodes from all seasons")]
    all: bool,
}

pub const CHARACTERS: [&'static str; 8] = [
    "Fry",
    "Leela",
    "Bender",
    "Prof. Farnsworth",
    "Zoidberg",
    "Hermes",
    "Amy",
    "Zapp Brannigan",
];
