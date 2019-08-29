mod aws;
mod commands;

use clap::{load_yaml, App};
use commands::s3::s3_commands;
use commands::ssm::ssm_commands;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("ssm", Some(matches)) => ssm_commands(matches),
        ("s3", Some(matches)) => s3_commands(matches),
        _ => (),
    }
}
