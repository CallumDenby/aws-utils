use clap::ArgMatches;

use rusoto_core::Region;
use rusoto_core::credential::ProfileProvider;
use rusoto_core::request::HttpClient;

use rusoto_ssm::SsmClient;

pub fn get_ssm_client(matches: &ArgMatches) -> SsmClient {
  let http_client = HttpClient::new().unwrap();

  let credentials = get_credentials(matches);

  SsmClient::new_with(http_client, credentials, Region::default())
}

pub fn get_credentials(matches: &ArgMatches) -> ProfileProvider {
  let mut credentials = ProfileProvider::new().unwrap();
  if matches.is_present("profile") {
      credentials.set_profile(matches.value_of("profile").unwrap());
  }

  credentials
}