use clap::{ Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Episodes(Episodes),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = "Get list of episodes from series or season")]
pub struct Episodes{
    name: Option<String>,
    #[arg(short, long, help="Season number (1-7)", value_name="int")]
    season: Option<i32>,
    #[arg(short, long, help="Show episodes from all seasons")]
    all: bool,
}


