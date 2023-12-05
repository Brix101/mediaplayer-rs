use clap::{Arg, Command};

#[derive(Debug)]
pub struct Args {
    pub player: Option<String>,
}

impl Args {
    // Parse command-line arguments and return an instance of Args
    pub fn parse() -> Self {
        let cmd = Command::new("Mediaplayer")
            .version("0.0.1")
            .about("A simple Rust CLI tool")
            .arg(
                Arg::new("player")
                    .short('P')
                    .long("player")
                    .help("Query the media player name"),
            )
            .get_matches();

        let player = cmd.get_one::<String>("player");
        match player {
            Some(player) => Self {
                player: Some((*player).to_owned()),
            },
            None => Self { player: None },
        }
    }
}
