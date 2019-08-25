mod search;

use clap::ArgMatches;
use search::ssm_search;

pub fn ssm_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        ("search", Some(matches)) => ssm_search(matches),
        _ => (),
    }
}
