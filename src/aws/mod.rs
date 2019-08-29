use clap::ArgMatches;

use std::str::FromStr;

use rusoto_core::credential::ProfileProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;

use rusoto_s3::S3Client;
use rusoto_ssm::SsmClient;

pub fn get_ssm_client(matches: &ArgMatches) -> SsmClient {
    let http_client = HttpClient::new().unwrap();

    let credentials = get_credentials(matches);

    let region = get_region(matches);

    SsmClient::new_with(http_client, credentials, region)
}

pub fn get_s3_client(matches: &ArgMatches) -> S3Client {
    let http_client = HttpClient::new().unwrap();

    let credentials = get_credentials(matches);

    let region = get_region(matches);

    S3Client::new_with(http_client, credentials, region)
}

pub fn get_region(matches: &ArgMatches) -> Region {
    Region::from_str(matches.value_of("region").unwrap_or("")).unwrap_or(Region::default())
}

pub fn get_credentials(matches: &ArgMatches) -> ProfileProvider {
    let mut credentials = ProfileProvider::new().unwrap();
    if matches.is_present("profile") {
        credentials.set_profile(matches.value_of("profile").unwrap());
    }

    credentials
}
