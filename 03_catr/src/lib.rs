use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file(s).", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number",
        help = "Number all output lines",
        default_value_t = false,
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonempty output lines",
        default_value_t = false,
        conflicts_with = "number_lines"
    )]
    number_nonblank_lines: bool,
}

struct Config {
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);
    let config = Config {
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    };

    for filename in args.files {
        let file = match open(&filename) {
            Ok(file) => Some(file),
            Err(err) => {
                eprintln!("Failed to open {}: {}", filename, err);
                None
            }
        };

        if let Some(f) = file {
            cat(f, &config)?;
        }
    }

    Ok(())
}

fn cat(f: Box<dyn BufRead>, config: &Config) -> MyResult<()> {
    let mut n_blank = 0;

    for (i, line_result) in f.lines().enumerate() {
        let line = line_result?;

        if config.number_lines {
            println!("{:indent$}\t{}", i + 1, line, indent = 6)
        } else if config.number_nonblank_lines {
            if line.is_empty() {
                n_blank += 1;
                println!("");
            } else {
                println!("{:indent$}\t{}", i + 1 - n_blank, line, indent = 6)
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
