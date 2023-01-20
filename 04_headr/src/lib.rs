use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file(s).", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        help = "print the first NUM lines",
        default_value_t = 10,
        conflicts_with = "bytes"
    )]
    lines: usize,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "print the first NUM bytes of each file",
        conflicts_with = "lines"
    )]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    if let Some(bytes) = args.bytes {
        if !(bytes > 0) {
            eprintln!("Number of bytes must be a positive integer.");
            std::process::exit(1);
        }
    } else {
        if !(args.lines > 0) {
            eprintln!("Number of lines must be a positive integer.");
            std::process::exit(1);
        }
    }

    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn head_bytes(file: Box<dyn BufRead>, n_bytes: usize) -> MyResult<()> {
    let mut buffer = vec![0; n_bytes];
    let mut handle = file.take(n_bytes as u64);
    let n_bytes_read = handle.read(&mut buffer)?;

    print!("{}", String::from_utf8_lossy(&buffer[..n_bytes_read]));

    Ok(())
}

fn head_lines(mut file: Box<dyn BufRead>, n_lines: usize) -> MyResult<()> {
    let mut line = String::new();

    for _ in 0..n_lines {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    Ok(())
}

pub fn run(args: Args) -> MyResult<()> {
    let n_files = args.files.len();

    for (i, filename) in args.files.iter().enumerate() {
        if n_files > 1 {
            println!("==> {} <==", filename);
        }

        match open(&filename) {
            Ok(file) => match args.bytes {
                Some(bytes) => head_bytes(file, bytes)?,
                None => head_lines(file, args.lines)?,
            },
            Err(err) => {
                eprintln!("Failed to open {}: {}", filename, err);
            }
        };

        if n_files > 1 && (i + 1) < n_files {
            println!("");
        }
    }

    Ok(())
}
