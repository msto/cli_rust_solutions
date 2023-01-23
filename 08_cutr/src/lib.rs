// use crate::Extract::*;
use clap::Parser;
use regex::{Captures, Regex};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file(s)", id = "FILE", default_value = "-")]
    paths: Vec<String>,

    #[arg(
        short = 'b',
        long = "bytes",
        help = "Select only these bytes",
        conflicts_with = "characters",
        conflicts_with = "fields"
    )]
    bytes: Option<String>,

    #[arg(
        short = 'c',
        long = "chars",
        help = "Select only these characters",
        conflicts_with = "fields",
        conflicts_with = "bytes"
    )]
    characters: Option<String>,

    #[arg(
        short = 'f',
        long = "fields",
        help = "Select only these fields",
        conflicts_with = "bytes",
        conflicts_with = "characters"
    )]
    fields: Option<String>,

    #[arg(
        short = 'd',
        long = "delim",
        id = "DELIM",
        help = "Field delimiter",
        default_value_t = '\t'
    )]
    delimiter: char,
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

pub fn run(args: Args) -> MyResult<()> {
    dbg!(args);

    Ok(())
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    Ok(args)
}

fn _open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_range(s: &str) -> Result<Range<usize>, String> {
    let single = Regex::new(r"^(\d+)$").unwrap();
    let multi = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let start: usize;
    let end: usize;

    let extract_pos = |caps: &Captures, idx: usize| -> usize {
        caps.get(idx)
            .map_or("", |x| x.as_str())
            .parse::<usize>()
            .unwrap()
    };

    if single.is_match(s) {
        let caps = single.captures(s).unwrap();
        start = extract_pos(&caps, 1);
        end = start + 1;
    } else if multi.is_match(s) {
        let caps = multi.captures(s).unwrap();
        start = extract_pos(&caps, 1);
        end = extract_pos(&caps, 2);
    } else {
        return Err(format!("Invalid numeric range: {}", s));
    }

    match (start, end) {
        (0, _) => Err("illegal list value: \"0\"".to_string()),
        (_, 0) => Err("illegal list value: \"0\"".to_string()),
        (_, _) => Ok(Range {
            start: start,
            end: end,
        }),
    }
}

fn parse_pos(s: &str) -> MyResult<PositionList> {
    s.split(',')
        .into_iter()
        .map(|r| parse_range(r))
        .collect::<Result<PositionList, _>>()
        .map_err(From::from) // TODO: I don't understand what this does or why it's necessary to compile
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;
    use super::parse_range;

    #[test]
    fn test_parse_pos() {
        //The empty string is an error
        // assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("1-2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![1..2]);
    }

    #[test]
    fn test_parse_range() {
        assert!(parse_range("").is_err());

        let res = parse_range("0");
        assert!(res.is_err());

        let res = parse_range("1-2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1..2);
    }
}
