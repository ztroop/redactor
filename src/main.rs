extern crate clap;
use clap::{App, Arg};

fn main() {
    // Set up the CLI parser
    let matches = App::new("Redactor")
        .version("1.0")
        .author("Author: Zackary Troop")
        .about("Description: Redact all the things!")
        .arg(Arg::with_name("input")
            .help("The string to redact")
            .required(true)
            .index(1))
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .help("Specify the length to redact. If not provided, the full length of the string is used")
            .takes_value(true))
        .arg(Arg::with_name("redact_char")
            .short("r")
            .long("redact-char")
            .help("Character to use for redaction. Default is '*'")
            .takes_value(true))
        .arg(Arg::with_name("ignore_char")
            .short("i")
            .long("ignore-char")
            .help("Character to ignore during redaction")
            .takes_value(true))
        .arg(Arg::with_name("ignore_last")
            .short("e")
            .long("ignore-last")
            .help("Ignore the last X characters from redaction")
            .takes_value(true))
        .arg(Arg::with_name("ignore_first")
            .short("f")
            .long("ignore-first")
            .help("Ignore the first X characters from redaction")
            .takes_value(true))
        .get_matches();

    // Get the input string
    let input = matches.value_of("input").unwrap().to_string();

    // Get the length to redact
    let length = matches
        .value_of("length")
        .map_or(input.len(), |v| v.parse::<usize>().unwrap_or(input.len()));

    // Get the redact character
    let redact_char = matches
        .value_of("redact_char")
        .unwrap_or("*")
        .chars()
        .next()
        .unwrap();

    // Get the ignore character
    let ignore_char = matches
        .value_of("ignore_char")
        .map(|c| c.chars().next().unwrap());

    // Get the number of characters to ignore at the end
    let ignore_last = matches
        .value_of("ignore_last")
        .map_or(0, |v| v.parse::<usize>().unwrap_or(0));

    // Get the number of characters to ignore at the start
    let ignore_first = matches
        .value_of("ignore_first")
        .map_or(0, |v| v.parse::<usize>().unwrap_or(0));

    // Do the redaction
    let redacted_str: String = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i < ignore_first || i >= input.len() - ignore_last {
                c
            } else if Some(c) == ignore_char {
                c
            } else if i < length {
                redact_char
            } else {
                c
            }
        })
        .collect();

    // Print the redacted string
    println!("{}", redacted_str);
}
