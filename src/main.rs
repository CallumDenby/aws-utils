mod app;
mod aws;
mod commands;

use app::build_cli;
use commands::s3::s3_commands;
use commands::ssm::ssm_commands;

fn main() {
    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("ssm", Some(matches)) => ssm_commands(matches),
        ("s3", Some(matches)) => s3_commands(matches),
        _ => (),
    }
}
