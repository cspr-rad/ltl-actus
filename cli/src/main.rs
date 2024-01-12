use std::{fs, path::PathBuf};

use structopt::StructOpt;
use toml;

use actus::pam::PamTerms;

/// ACTUS in LTL - demo cli
#[derive(StructOpt, Debug)]
#[structopt(name = "ltl-actus")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(long = "verbose", global = true)]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Executes an instrument
    Exec {
        #[structopt(long = "path")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::from_args();

    match cli.cmd {
        Command::Exec { path } => {
            let contents = fs::read_to_string(path).expect("Unable to read file");
            let terms: PamTerms = toml::from_str::<PamTerms>(&contents).unwrap();
            println!("terms: {:?}", terms);
            println!("dummy implementation");
        }
    }
}
