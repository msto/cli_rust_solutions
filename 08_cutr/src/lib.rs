// use crate::Extract::*;
use clap::Parser;
use regex::{Captures, Regex};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
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

fn parse_idx(s: &str) -> Result<usize, String> {
    let value_err = || format!("illegal list value: \"{}\"", s);

    s.starts_with('+')
        .then(|| Err(value_err()))
        .unwrap_or_else(|| {
            s.parse::<NonZeroUsize>()
                .map(|x| usize::from(x) - 1)
                .map_err(|_| value_err())
        })
}

fn parse_range(s: &str) -> Result<Range<usize>, String> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();

    let extract_range = |caps: Captures| -> Result<Range<usize>, String> {
        let start = parse_idx(&caps[1])?;
        let end = parse_idx(&caps[2])?;
        if start >= end {
            return Err(format!(
                "First number in range ({}) must be lower than second number ({})",
                start + 1,
                end + 1
            ));
        }

        Ok(start..end + 1)
    };

    match range_re.captures(s) {
        Some(caps) => extract_range(caps),
        None => Err(format!("illegal list value: \"{}\"", s)),
    }
}

fn parse_pos(s: &str) -> MyResult<PositionList> {
    s.split(',')
        .into_iter()
        // .map(|r| parse_range(r))
        .map(|r| parse_idx(r).map(|x| x..x + 1).or_else(|_| parse_range(r)))
        .collect::<Result<PositionList, _>>()
        .map_err(From::from) // TODO: I don't understand what this does or why it's necessary to compile
}

#[cfg(test)]
mod unit_tests {
    use super::parse_idx;
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

        // Leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"");

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"");

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-+2\"");

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"");

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"");

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-a\"");

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a-1\"");

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_parse_range() {
        assert!(parse_range("").is_err());

        let res = parse_range("0");
        assert!(res.is_err());

        let res = parse_range("1-2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0..2);
    }

    #[test]
    fn test_parse_idx() {
        assert!(parse_idx("").is_err());

        let res = parse_idx("0");
        assert!(res.is_err());

        let res = parse_idx("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
    }
}
