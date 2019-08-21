mod search;

use search::ssm_search;
use clap::ArgMatches;

pub fn ssm_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        ("search", Some(matches)) => ssm_search(matches),
        _ => ()
    }
}
