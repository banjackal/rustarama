use clap::{ Subcommand, Args};

#[path = "./infosphere.rs"]
mod infosphere;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command()]
    Episodes(Episodes),
    #[command()]
    Characters(Characters),
    #[command()]
    Quote(Quote),
}

pub const CHARACTERS: &str = "
    Fry
    Leela
    Bender
    Prof. Farnsworth
    Zoidberg
    Hermes
    Amy
    Zapp Brannigan";

#[derive(Args, Debug)]
pub struct Characters{}

impl Characters {
    pub fn print(){
        println!("{}", CHARACTERS)
    }
}

#[derive(Args, Debug, Clone)]
#[command(author, version, about, long_about = "Get list of episodes from series or season")]
pub struct Episodes {
    name: Option<String>,
    #[arg(short, long, value_parser(clap::value_parser!(i32).range(1..8)), help="Season number (1-8)", value_name="int")]
    season: Option<i32>,
}

impl Episodes {
    pub fn get_episodes(&self) {
        if let Some(season) = self.season {
            infosphere::get_episodes(Some(season)).unwrap()
        }
        else {
            infosphere::get_episodes(None).unwrap()
        }
        
    }
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Get random Futurama quote")]
pub struct Quote {
    #[arg(short, long, help="Character name (e.g. 'Fry', 'Bender'")]
    pub character: Option<String>,
    #[arg(short, long, help="Episode name (use 'futurama get episodes' command for assistance)")]
    pub episode: Option<String>,
    #[arg(short, conflicts_with("episode"), long, value_parser(clap::value_parser!(i32).range(1..8)),help="Season number (1-8)", value_name="int")]
    pub season: Option<i32>,
    #[arg(requires("episode"),conflicts_with("character"), conflicts_with("season"),short, long, help="Toggle for returning all quotes from an episode")]
    all: bool,
}

impl Quote {
    pub fn get_quote(&self) {
        if self.all {
            if let Some(episode) = &self.episode {
                infosphere::print_all_quotes_from_episode(&episode).unwrap();
            }
        }
        else {
            infosphere::get_quote(&self).unwrap();
        }
    } 
}

