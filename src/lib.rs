extern crate regex;

use regex::Regex;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn env(expression: &str, env_files: &[&str]) -> Option<String> {
    let mut result: Option<String> = None;

    for env_file in env_files {
        if let Ok(f) = File::open(env_file) {
            let file = BufReader::new(f);

            let regex = Regex::new(expression).unwrap();

            for line in file.lines() {
                let l = line.unwrap();

                if regex.is_match(&l) {
                    for cap in regex.captures_iter(&l) {
                        result = Some(cap[1].to_string());
                    }
                }
            }
        }
    }

    result
}
