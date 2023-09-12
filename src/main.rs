extern crate clap;
use clap::{App, Arg};

fn main() {
    // Set up the CLI parser
    let matches = App::new("Redactor")
        .version("1.0")
        .author("Author: Zackary Troop <zackary.troop@outlook.com>")
        .about("Description: A utility to redact strings")
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
    let redacted_str: String = redact(
        &input,
        length,
        redact_char,
        ignore_char,
        ignore_last,
        ignore_first,
    );

    // Print the redacted string
    println!("{redacted_str}");
}

fn redact(
    input: &str,
    length: usize,
    redact_char: char,
    ignore_char: Option<char>,
    ignore_last: usize,
    ignore_first: usize,
) -> String {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i < ignore_first || i >= input.len() - ignore_last {
                c
            } else if Some(c) == ignore_char {
                c
            } else if i >= ignore_first && i < ignore_first + length {
                redact_char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn test_redact_full_length() {
        let result = redact("abcdef", 6, '*', None, 0, 0);
        assert_eq!(result, "******");
    }

    #[test]
    fn test_redact_partial_length() {
        let result = redact("abcdef", 3, '*', None, 0, 0);
        assert_eq!(result, "***def");
    }

    #[test]
    fn test_redact_ignore_first() {
        let result = redact("abcdef", 6, '*', None, 0, 2);
        assert_eq!(result, "ab****");
    }

    #[test]
    fn test_redact_ignore_last() {
        let result = redact("abcdef", 6, '*', None, 2, 0);
        assert_eq!(result, "****ef");
    }

    #[test]
    fn test_redact_ignore_char() {
        let result = redact("abcdef", 6, '*', Some('c'), 0, 0);
        assert_eq!(result, "**c***");
    }

    #[test]
    fn test_redact_empty_string() {
        let result = redact("", 6, '*', None, 0, 0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_redact_ignore_first_and_last() {
        let result = redact("abcdef", 4, '*', None, 1, 1);
        assert_eq!(result, "a****f");
    }
}
