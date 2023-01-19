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

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);

    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            // Ok(_) => println!("Opened {}", filename),
            Ok(f) => cat(f),
        }
    }

    Ok(())
}

fn cat(f: Box<dyn BufRead>) {
    for line in f.lines() {
        println!("{}", line.unwrap())
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
