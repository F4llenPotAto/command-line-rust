use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(contents) => {
                match config.number_lines {
                    true => {
                        let mut line_count = 1;
                        for line in contents.lines() {
                            println!("{} {}", line_count, line.unwrap());
                            line_count += 1;
                        }
                    }
                    false => {
                        for line in contents.lines() {
                            println!("{}", line.unwrap());
                        }
                    }
                }
                match config.number_nonblank_lines {
                    true => {
                        let mut line_count = 1;
                        for line in contents.lines() {
                            match line.expect("Failed to read line").as_ref() {
                                "\n" => {
                                    println!("{}", line_count);
                                    line_count += 1;
                                }
                                _ => {
                                    println!("{} {:#?}", line_count, line);
                                    line_count += 1;
                                }
                            }
                        }
                    }
                    false => {
                        for line in contents.lines() {
                            println!("{}", line.unwrap());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .about("Trenton Ornelas <trentonornelas@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Only print nonblank lines.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank")
                .help("Print lines with numbers."),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
