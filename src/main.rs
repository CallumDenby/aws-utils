mod aws;
mod commands;

use clap::{load_yaml, App};
use commands::ssm::ssm_commands;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("ssm", Some(matches)) => ssm_commands(matches),
        _ => (),
    }
}
