use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		// error messaging
		if args.len() < 3 {
			return Err("Not enough arguments.");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive: bool;

		// let case_sensitive = &false;
		if args.len() == 3 {
			if args[2] == "-e" {
				case_sensitive = true;
			} else{
				case_sensitive = false;
			}
		} else {
		    case_sensitive = false;
		}

		Ok(Config {
			query: query,
			filename: filename,
			case_sensitive: case_sensitive
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
	println!("Searching for: {}", config.query );
	println!("In file: {}", config.filename);

	let mut f = File::open(config.filename)?;
	let mut contents = String::new();

	f.read_to_string(&mut contents)?;

	let results = if config.case_sensitive == true {
	    search(&config.query, &contents)
	} else {
	    search_case_insensitive(&config.query, &contents)
	};

	for line in results {
	    println!("{}", line);
	}

	Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // add any matches to vector
        if line.contains(query) {
            results.push(line);
        }
    }
    // add messaging if no results are found
    if results.len() < 1 {
    	results.push("****************\nNO MATCHES FOUND\n****************");
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query2 = query.to_lowercase();

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query2) {
            results.push(line);
        }
    }

    if results.len() < 1 {
        results.push("****************\nNO MATCHES FOUND\n****************");
    }

    results
}

// *********************************************************************************************
// test cases below
// *********************************************************************************************

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn one_result() {
	    let query = "duct";
	    let contents = "\
Rust:
safe, fast, productive.
Pick three!";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}

	#[test]
	fn case_sensitive() {
	    let query = "duct";
	    let contents = "\
Rust:
safe, fast, productive.
Pick three!";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents) 
		);
	}

	#[test]
	fn case_insensitive() {
	    let query = "dUct";
	    let contents = "\
Rust:
safe, fast, productive.
Pick three!";

		assert_eq!(
			vec!["safe, fast, productive."],
			search_case_insensitive(query, contents) 
		);
	}
}