mod delete;

use clap::ArgMatches;
use delete::s3_delete;

pub fn s3_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        ("delete", Some(matches)) => s3_delete(matches),
        _ => (),
    }
}
