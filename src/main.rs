mod commands;
mod aws;

use commands::ssm::ssm_commands;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .get_matches();

    match matches.subcommand() {
        ("ssm", Some(matches)) => ssm_commands(matches),
        _ => ()
    }
}
