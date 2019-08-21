use std::{thread, time};
use clap::ArgMatches;
use time::Duration;

use rusoto_ssm::{SsmClient, Ssm, ParameterMetadata, DescribeParametersRequest};

use crate::aws::get_ssm_client;

pub fn ssm_fetch_params(client: &SsmClient, next_token: String) -> Option<(String, Vec<ParameterMetadata>)> {
    let describe_request = DescribeParametersRequest {
        filters: None,
        max_results: Some(50),
        next_token: if next_token.len() > 0 { Some(next_token) } else { None },
        parameter_filters: None
    };  

    match client.describe_parameters(describe_request).sync() {
        Ok(output) => {
            match output.next_token {
                Some(next_token) => Some((next_token, output.parameters.unwrap())),
                None => None
            }
        },
        Err(err) => {
            eprintln!("Error Describing {:?}", err);
            None
        }
    }
}

pub fn ssm_search(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap_or("");

    let client = get_ssm_client(matches);

    let mut results = Vec::new();
    let mut next_token = String::new();

    'main: loop {
        let response: Vec<ParameterMetadata> = match ssm_fetch_params(&client, next_token) {
            Some((token, parameters)) => {
                next_token = token;
                parameters.iter().filter(|parameter| {
                    let name = parameter.name.as_ref().unwrap();

                    name.contains(input)
                }).cloned().collect()
            },
            None => break 'main
        };
        results.extend(response);

        thread::sleep(Duration::from_millis(100));
    }

    for result in results.iter() {
        println!("{}", result.name.as_ref().unwrap())
    }
}
