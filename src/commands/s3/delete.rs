use clap::ArgMatches;

use crate::aws::get_s3_client;
use rusoto_s3::{
    Delete, DeleteBucketRequest, DeleteObjectsRequest, ListObjectsV2Request, Object,
    ObjectIdentifier, S3Client, S3,
};

pub fn s3_list_objects(
    client: &S3Client,
    bucket: &str,
    next_token: String,
) -> Option<(String, Vec<Object>)> {
    let list_request = ListObjectsV2Request {
        bucket: bucket.to_owned(),
        continuation_token: if next_token.len() > 0 {
            Some(next_token)
        } else {
            None
        },
        delimiter: None,
        encoding_type: None,
        fetch_owner: None,
        max_keys: None,
        prefix: None,
        request_payer: None,
        start_after: None,
    };

    match client.list_objects_v2(list_request).sync() {
        Ok(output) => {
            println!("Output {:?}", output);
            if output.key_count.unwrap() > 0 {
                return Some((
                    output.next_continuation_token.unwrap_or(String::new()),
                    output.contents.unwrap(),
                ));
            }
            None
        }
        Err(err) => {
            eprintln!("Error Listing Object {:?}", err);
            None
        }
    }
}

pub fn s3_delete_objects(client: &S3Client, bucket: &str, objects: Vec<ObjectIdentifier>) {
    let delete_request = DeleteObjectsRequest {
        bucket: bucket.to_owned(),
        bypass_governance_retention: None,
        delete: Delete {
            objects: objects,
            quiet: Some(true),
        },
        mfa: None,
        request_payer: None,
    };

    match client.delete_objects(delete_request).sync() {
        Ok(output) => println!("Deleted {:?}", output),
        Err(err) => eprintln!("Error deleting object {:?}", err),
    };
}

pub fn s3_delete_bucket(client: &S3Client, bucket: &str) {
    let delete_request = DeleteBucketRequest {
        bucket: bucket.to_owned(),
    };

    match client.delete_bucket(delete_request).sync() {
        Ok(()) => println!("Bucket deleted"),
        Err(err) => {
            eprintln!("Error deleting bucket {:?}", err);
        }
    }
}

pub fn s3_delete(matches: &ArgMatches) {
    println!("Running s3 delete");
    let input = matches.value_of("input").unwrap();

    println!("Got input {}", input);

    let client = get_s3_client(matches);

    let mut next_token = String::new();

    match s3_list_objects(&client, input, next_token) {
        Some((token, contents)) => {
            next_token = token;

            if matches.is_present("force") {
                let response = contents
                    .into_iter()
                    .map(|item| ObjectIdentifier {
                        key: item.key.unwrap(),
                        version_id: None,
                    })
                    .collect();
                s3_delete_objects(&client, input, response);

                while let Some((token, contents)) = s3_list_objects(&client, input, next_token) {
                    next_token = token;
                    let response = contents
                        .into_iter()
                        .map(|item| ObjectIdentifier {
                            key: item.key.unwrap(),
                            version_id: None,
                        })
                        .collect();

                    s3_delete_objects(&client, input, response);
                }
            } else {
                println!("Items exist in bucket, run with --force to delete");
            }
        }
        None => {
            println!("No items found in bucket");
            s3_delete_bucket(&client, input);
        }
    }

    // while let Some((token, contents)) = s3_list_objects(&client, input, next_token) {
    //   println!("Got some output");
    //   next_token = token;
    //   let response = contents.into_iter().map(|item| ObjectIdentifier {
    //     key: item.key.unwrap(),
    //     version_id: None
    //   }).collect();

    //   println!("Deleting {:?}", response);
    //   s3_delete_objects(&client, input, response);
    // }
    // println!("No output here");
}
