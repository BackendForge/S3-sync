extern crate s3;
extern crate serde_json;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;

use chrono::prelude::*;
use dotenv::dotenv;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

const BUCKET_LIST_TIMEOUT: Option<Duration> = Some(Duration::new(900, 0));

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

/// Saves input string to file in the given path.
///
/// # Arguments
/// * `data` - HashMap of data to be saved.
/// * `bucket_name` - Suffix of file name to save the data to. Prefix is: rados_copy_details_
/// * `path_to_file_dir` - Path to directory where the file will be saved.
///
/// # Returns
/// * `Result` - Result of saving data to file. On success, `Ok` is returned. On failure, `Err` is returned.
///
/// # Errors
/// * `Box<dyn Error>` - Error while saving data to file.
///
/// # Example
/// ```rust,no_run
/// use rados-list::save_hashmap_to_file;
///
/// let data: HashMap<String, u64> = HashMap::new();
/// let bucket_name = "test_bucket";
/// let path_to_file_dir = "./";
///
/// save_hashmap_to_file(&data, bucket_name, path_to_file_dir);
/// ```
fn save_hashmap_to_file(
    data: &HashMap<String, u64>,
    bucket_name: &str,
    path_to_file_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!(
            "{}/rados_copy_details_{}",
            path_to_file_dir, bucket_name
        ))?;
    // file.write_all(data.as_bytes())?;
    serde_json::to_writer(&file, &data)?;
    Ok(())
}

#[test]
fn test_save_hashmap_to_file() {
    let data: HashMap<String, u64> = HashMap::new();
    let bucket_name = "test_bucket";
    let path_to_file_dir = "./";
    save_hashmap_to_file(&data, bucket_name, path_to_file_dir).unwrap();
}

/// Saves input string to file in the given path.
///
/// # Arguments
/// * `data` - String to be saved to file.
/// * `path_to_file_dir` - Path to directory where file will be saved.
/// * `file_name` - Name of file to be saved.
/// * `truncate` - If true, file will be truncated before writing.
///
/// # Returns
/// * `Result` - Result of saving data to file. On success, `Ok` is returned. On failure, `Err` is returned.
///
/// # Errors
/// * `Box<dyn Error>` - Error while saving data to file.
///
/// # Example
/// ```rust,no_run
/// use rados-list::save_to_file;
///
/// let data = "Hello World!";
/// let path_to_file_dir = "./";
/// let file_name = "test.txt";
///
/// save_str_to_file(data, path_to_file_dir, file_name, true);
/// ```
fn save_str_to_file(
    data: &str,
    file_name: &str,
    path_to_file_dir: &str,
    truncate: bool,
) -> Result<(), Box<dyn Error>> {
    let append = !truncate;
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(truncate)
        .create(true)
        .open(format!("{}/{}", path_to_file_dir, file_name))?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

#[test]
fn test_save_str_to_file() {
    let data = "bucket_name";
    let file_name = "rados_copy_details_test";
    let path_to_file_dir = "./";
    save_str_to_file(data, file_name, path_to_file_dir, false).unwrap();
}

/// Reads given file and returns contents as String.
fn read_from_file_to_str(path_to_file: &str) -> String {
    std::fs::read_to_string(path_to_file).unwrap()
}

/// Saves input string to file in the given path. Requires data to be 0 or 1.
///
/// # Arguments
/// * `data` - String to be saved to file.
/// * `path_to_file_dir` - Path to directory where file will be saved.
/// * `file_name` - Name of file to be saved.
///
/// # Returns
/// * `Result` - Result of saving data to file. On success, `Ok` is returned. On failure, `Err` is returned.
///
/// # Errors
/// * `Box<dyn Error>` - Error while saving data to file.
///
/// # Example
/// ```rust,no_run
/// use rados-list::save_status_to_file;
///
/// let data = "1";
/// let path_to_file_dir = "./";
/// let file_name = "test.txt";
///
/// save_status_to_file(data, path_to_file_dir, file_name, true);
/// ```
fn save_status_to_file(
    data: &str,
    file_name: &str,
    path_to_file_dir: &str,
) -> Result<(), Box<dyn Error>> {
    if data == "0" {
        let file_exist = Path::new(&format!("{}/{}", path_to_file_dir, file_name)).exists();
        if !file_exist {
            save_str_to_file(data, file_name, path_to_file_dir, true)?;
        }
    } else if data == "1" {
        save_str_to_file(data, file_name, path_to_file_dir, true)?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid status for save_status_to_file",
        )));
    }
    Ok(())
}

#[test]
fn test_save_status_to_file() {
    let data = "1";
    let file_name = "rados_copy_status_test";
    let path_to_file_dir = "./";
    save_status_to_file(data, file_name, path_to_file_dir).unwrap();
    let data = "0";
    let file_name = "rados_copy_status_test";
    let path_to_file_dir = "./";
    save_status_to_file(data, file_name, path_to_file_dir).unwrap(); // this value should not overwrite the file
    let file_data = read_from_file_to_str(&format!("{}/{}", path_to_file_dir, file_name));
    assert_eq!(file_data, "1");
}


/// Function that compares two dates and returns false if current_datetime is greater than maximum_datetime.
/// 
/// # Arguments
/// * `current_datetime` - Current datetime.
/// * `maximum_datetime` - Maximum datetime.
/// 
/// # Returns
/// * `bool` - Returns true if current_datetime is less than or equal to maximum_datetime.
/// 
/// # Example
/// ```rust,no_run
/// use rados-list::compare_dates;
/// 
/// let current_datetime = "2020-01-01T00:00:00";
/// let maximum_datetime = "2020-01-01T00:00:00";
/// 
/// let result = compare_dates(current_datetime, maximum_datetime);
/// assert_eq!(result, true);
/// ```
fn compare_dates(
    current_datetime: String,
    maximum_datetime: DateTime<FixedOffset>,
) -> bool {
    current_datetime.parse::<DateTime<FixedOffset>>().unwrap() <= maximum_datetime
}

#[test]
fn test_compare_dates() {
    let current_datetime = String::from("2023-07-21T12:28:10.490Z");
    let maximum_datetime = DateTime::parse_from_rfc3339("2044-11-28T21:00:09+09:00").unwrap();
    let result = compare_dates(current_datetime, maximum_datetime);
    assert_eq!(result, true);
}

/// Connects to S3 and retrieves list of objects in buckets.
/// List of buckets has to be provided in file which full path is provided as input.
///
/// During execution it requires present of folder defined in `.env`: PATH_TO_FILE_DIR in the current directory.
/// Program will save multiple files to this folder:
/// * `rados_copy_details_<bucket_name>` - File with list of objects in `bucket` which are different between two storage providers.
/// * `rados_copy_details` - File with list of buckets with not same objects.
/// * `rados_copy_status` - File with status of execution. If all objects are the same on both providers, it will contain 0, otherwise 1.
///
/// # Returns
/// * `Result` - Result of retrieving list of objects in buckets. On success, `Ok` is returned. On failure, `Err` is returned.
///
/// # Errors
/// * `S3Error` - Error while retrieving list of objects in buckets.
///
#[tokio::main]
async fn main() -> Result<(), S3Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <bucket list file path> <file with maximum datetime>",
            args[0]
        );
        panic!(
            "Example: {} /app/bucket_list/bucket_list_1.txt /app/datetime/datetime.txt",
            args[0]
        );
    }
    dotenv().expect("Failed to read .env file");
    let path_to_file_dir: &str = &env::var("PATH_TO_FILE_OUTPUT_DIR").unwrap();
    let path_to_datetime_file: &str = &args[2];
    if !Path::new(path_to_file_dir).exists() {
        panic!("{} does not exist", path_to_file_dir);
    }
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let buckets = contents.split("\n").collect::<Vec<&str>>();

    let mut src_dst_not_eq: &str = "0";
    let mut buckets_not_eq: Vec<&str> = Vec::new();

    for one_bucket in buckets {
        let datetime_file_string = read_from_file_to_str(path_to_datetime_file);
        let maximum_datetime = DateTime::parse_from_rfc3339(&datetime_file_string).unwrap();
        let bucket_name = one_bucket.trim();
        if bucket_name.is_empty() {
            continue;
        }

        let ceph_a = Storage {
            name: "s3".into(),
            // region: "eu-central-1".parse()?,
            region: Region::Custom {
                region: "".into(),
                endpoint: env::var("CEPH_A_ADDRESS").unwrap().into(),
            },
            credentials: Credentials::from_env_specific(
                Some("CEPH_A_ACCESS_KEY_ID"),
                Some("CEPH_A_SECRET_ACCESS_KEY"),
                None,
                None,
            )?,
            bucket: bucket_name.to_string(),
            location_supported: true,
        };
        let ceph_b = Storage {
            name: "s3".into(),
            region: Region::Custom {
                region: "".into(),
                endpoint: env::var("CEPH_B_ADDRESS").unwrap().into(),
            },
            credentials: Credentials::from_env_specific(
                Some("CEPH_B_ACCESS_KEY_ID"),
                Some("CEPH_B_SECRET_ACCESS_KEY"),
                None,
                None,
            )?,
            bucket: bucket_name.to_string(),
            location_supported: true,
        };

        let mut objects: HashMap<String, u64> = HashMap::new();

        for backend in vec![ceph_a, ceph_b] {
            println!(
                "Running: {}, list objects from {}",
                backend.name, backend.bucket
            );
            // Create Bucket in REGION for BUCKET
            let mut bucket = Bucket::new(&backend.bucket, backend.region, backend.credentials)?;
            bucket.request_timeout = BUCKET_LIST_TIMEOUT;
            bucket = bucket.with_path_style();

            let mut results = Vec::new();
            let mut continuation_token = None;
            loop {
                println!("list_bucket_result loop iteration for 5 000 000 objects");
                let (list_bucket_result, _) = bucket
                    .list_page(
                        "".to_string(),
                        None,
                        continuation_token,
                        None,
                        Some(5000000),
                    )
                    .await?;
                continuation_token = list_bucket_result.next_continuation_token.clone();
                results.push(list_bucket_result);
                if continuation_token.is_none() {
                    break;
                }
            }
            // list_bucket_result - filter objects via date, this `date` read from specific file
            // last_modified: String,
            // last_modified: "2022-07-21T12:28:10.490Z"
            results.retain(|x| {
                compare_dates(
                    x.contents.to_vec()[0].last_modified.clone(),
                    maximum_datetime,
                )
            });

            for result in results {
                let counter = result.contents.len();
                println!("{} objects found for one ListBucketResult", counter);

                for object in result.contents {
                    match objects.get(&object.key) {
                        // break
                        Some(_value) if *_value - object.size == 0 => objects.remove(&object.key),
                        None => objects.insert(object.key, object.size),
                        Some(_) => todo!(),
                    };
                }
            }
        } // end of for backend in vec![ceph_a, ceph_b]
        println!("objects different: {:?}", &objects.len());
        if objects.len() > 0 {
            save_hashmap_to_file(&objects, &bucket_name, path_to_file_dir).unwrap();
            buckets_not_eq.push(bucket_name);
            src_dst_not_eq = "1";
        }
    } // end of for one_bucket in buckets
    save_status_to_file(src_dst_not_eq, "rados_copy_status", path_to_file_dir).unwrap();
    save_str_to_file(
        &buckets_not_eq
            .iter()
            .map(|f| format!("{}\n", f))
            .collect::<String>(),
        "rados_copy_details",
        path_to_file_dir,
        false,
    )
    .unwrap(); // saves list of buckets with different objects as append to file
    Ok(())
}
