use clap::{Arg, ArgAction, Command};
use reqwest;
use std::fs::File;
use std::io::{self, BufRead};
use tokio;

async fn get_request(url: &str) {
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => {
            println!("{}: {}", url, response.status());
        }
        Err(error) => {
            println!("{}: {}", url, error);
        }
    }
}

async fn get_list(matches: &clap::ArgMatches) -> Result<(), String> {
    let mut list = Vec::new();
    let mut files_given = false;
    if let Some(input_file) = matches.get_one::<String>("input") {
        match File::open(input_file) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        list.push(line);
                    }
                }
                files_given = true;
            }
            Err(error) => {
                return Err(format!("Error opening file: {}", error));
            }
        }
    }
    if let Some(values) = matches.get_many::<String>("values") {
        for value in values.into_iter() {
            list.push(value.to_string());
        }
        files_given = true;
    }
    if files_given {
        let futures = list.iter().map(|url| get_request(url));
        futures::future::join_all(futures).await;
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            if let Ok(line) = line {
                let futures = line.split_whitespace().map(|url| get_request(url));
                futures::future::join_all(futures).await;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = Command::new("linkchecker")
        .version("1.0")
        .author("Sachin Iyer")
        .about("A CLI application to verify links")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("values")
                .action(ArgAction::Append)
                .value_name("VALUE"),
        )
        .get_matches();

    let list = get_list(&matches);
    if let Err(error) = list.await {
        println!("{}", error);
    }
}
