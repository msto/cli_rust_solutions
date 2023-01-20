use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Add, AddAssign};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file(s).", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'l',
        long = "lines",
        help = "Print the newline counts",
        default_value_t = false
    )]
    lines: bool,

    #[arg(
        short = 'w',
        long = "words",
        help = "Print the word counts",
        default_value_t = false
    )]
    words: bool,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Print the byte counts",
        default_value_t = false,
        conflicts_with = "chars"
    )]
    bytes: bool,

    #[arg(
        short = 'm',
        long = "chars",
        help = "Print the character counts",
        default_value_t = false,
        conflicts_with = "bytes"
    )]
    chars: bool,
}

impl Args {
    fn set_defaults(&mut self) {
        // if !self.lines && !self.words && !self.bytes && !self.chars {
        if [self.lines, self.words, self.bytes, self.chars]
            .iter()
            .all(|v| !v)
        {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
    n_lines: usize,
    n_words: usize,
    n_bytes: usize,
    n_chars: usize,
}

impl FileInfo {
    fn new() -> Self {
        Default::default()
    }
}

impl Add for FileInfo {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            n_lines: self.n_lines + other.n_lines,
            n_words: self.n_words + other.n_words,
            n_bytes: self.n_bytes + other.n_bytes,
            n_chars: self.n_chars + other.n_chars,
        }
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            n_lines: self.n_lines + other.n_lines,
            n_words: self.n_words + other.n_words,
            n_bytes: self.n_bytes + other.n_bytes,
            n_chars: self.n_chars + other.n_chars,
        }
    }
}

pub fn get_args() -> MyResult<Args> {
    let mut args = Args::parse();
    args.set_defaults();

    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);

    let mut totals = FileInfo::new();

    for filename in &args.files {
        let stats = match open(filename) {
            Ok(file) => Some(count(file)?),
            Err(err) => {
                eprintln!("{}: {}", filename, err);
                None
            }
        };

        if let Some(stats) = stats {
            print_stats(
                &stats, filename, args.lines, args.words, args.chars, args.bytes,
            );

            totals += stats;
        }
    }

    if args.files.len() > 1 {
        print_stats(
            &totals, "total", args.lines, args.words, args.chars, args.bytes,
        );
    }

    Ok(())
}

fn print_stats(
    stats: &FileInfo,
    filename: &str,
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
) {
    if chars && bytes {
        panic!("Cannot print both chars and bytes.")
    }

    let mut statline = String::new();
    if lines {
        statline += &format!("{:>8}", stats.n_lines);
    }
    if words {
        statline += &format!("{:>8}", stats.n_words);
    }
    if chars {
        statline += &format!("{:>8}", stats.n_chars);
    }
    if bytes {
        statline += &format!("{:>8}", stats.n_bytes);
    }

    if filename != "-" {
        statline += &format!(" {}", filename);
    }

    println!("{}", statline)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// fn print_stats(file: impl BufRead, _args: &Args) -> MyResult<()> {
//     let stats = count(file)?;

//     Ok(())
// }

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut n_lines = 0;
    let mut n_words = 0;
    let mut n_bytes = 0;
    let mut n_chars = 0;

    let mut line = String::new();

    loop {
        let n_bytes_read = file.read_line(&mut line)?;

        if n_bytes_read == 0 {
            break;
        }

        n_lines += 1;
        n_words += line.split_whitespace().count();
        n_bytes += line.len();
        n_chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        n_lines,
        n_words,
        n_bytes,
        n_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            n_lines: 1,
            n_words: 10,
            n_chars: 48,
            n_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
