use std::env;
use std::io::prelude::*;
use std::fs;
use std::io;

extern crate log;
use log::{trace, debug, info, warn, error};

#[derive(Debug)]
pub struct Options {
        pub pattern: String,
        pub filename: String,
        pub case_sensitive: bool,
}

impl Options {
        pub fn from(mut args: std::env::Args) -> Options {
                trace!("{:?}", args);
                // let args: Vec<String> = args.collect();
                trace!("The arguments is `{:?}`", args);
                if args.len() < 3 {
                        panic!("Too less arguments, at least two!");
                };
                args.next();
                Options {
                        pattern: args.next().unwrap(),
                        filename: args.next().unwrap(),
                        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
                }
                // \TODO check available of arguments
        }
}

/*  \brief search context in line
*/
fn is_matched_line(pattern: &str, line: &str) -> bool {
        return line.contains(pattern);
}

fn is_matched_line_omit_case(pattern: &str, line: &str) -> bool {
        return line.to_lowercase().contains(&pattern.to_lowercase());
}

pub fn run(options: &Options) {
        let f = fs::File::open(options.filename.clone()).unwrap();
        let mut reader = io::BufReader::new(f);
        let mut line = String::new();
        let mut line_number = 0;

        loop {
                let len = reader.read_line(&mut line).unwrap();
                trace!("Read {} bytes!", len);
                if len == 0 {
                        break;
                }
                line_number += 1;

                // handle line
                trace!("{}", line);
                if options.case_sensitive {
                        if is_matched_line(
                                &options.pattern,
                                &line) {
                                println!("{} - {}", line_number, line);
                        }
                } else {
                        if is_matched_line_omit_case(
                                &options.pattern,
                                &line) {
                                println!("{} - {}", line_number, line);
                        }
                }
                line.clear();
        }
}

// testing bellow

#[cfg(test)]
mod tests {

use std::env;

// test `is_matched_line`
        #[test]
        fn test_is_matched_line() {
                debug_assert!(super::is_matched_line(
                        "22",
                        "22334422"));
        }

        #[test]
        fn test_is_mtached_line2() {
                debug_assert!(false == super::is_matched_line(
                        "22",
                        "233233233"));
        }

// test `is_matched_line_omit_case`
        #[test]
        fn test_is_matched_line_omit_case() {
                debug_assert!(super::is_matched_line_omit_case(
                        "AbcD",
                        "cabCdDDxyz"));
        }

        #[test]
        fn test_is_matched_line_omit_case2() {
                debug_assert!(false == super::is_matched_line_omit_case(
                        "aBcd",
                        "acdbesxf"));
        }

// test `run`
        #[test]
        fn test_run() {
                super::run(
                        &super::Options {
                                pattern: String::from("111"),
                                filename: String::from("test.txt"),
                                case_sensitive: false});
        }

        #[should_panic]
        #[test]
        fn test_run2() {
                super::run(
                        &super::Options {
                                pattern: String::from("111"),
                                filename: String::from("xxx.txt"),
                                case_sensitive: true});
        }
}
