use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file.", default_value = "-")]
    in_file: String,

    // #[arg(help = "Output file.", default_value = "-")]
    #[arg(help = "Output file.")]
    out_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        help = "Prefix lines by the number of occurrences",
        default_value_t = false
    )]
    count: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);

    let mut fin = open(&args.in_file).map_err(|e| format!("{}: {}", args.in_file, e))?;
    let mut fout: Box<dyn Write> = match args.out_file {
        Some(fname) => Box::new(File::create(fname)?),
        None => Box::new(io::stdout()),
    };
    // let out_fname = args.out_file.unwrap();
    // let mut fout = create(&out_fname).map_err(|e| format!("{}: {}", &out_fname, e))?;

    let mut write_line = |text: &str, n: u64| -> MyResult<()> {
        if n > 0 {
            if args.count {
                write!(fout, "{:>4} {}", n, text)?;
            } else {
                write!(fout, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut prev = String::new();
    let mut n_obs: u64 = 0;
    loop {
        let bytes = fin.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != prev.trim_end() {
            write_line(&prev, n_obs)?;
            prev = line.clone();
            n_obs = 0;
        }

        n_obs += 1;
        line.clear();
    }

    write_line(&prev, n_obs)?;

    Ok(())
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// fn create(filename: &str) -> MyResult<Box<dyn Write>> {
//     match filename {
//         "-" => Ok(Box::new(io::stdout())),
//         _ => Ok(Box::new(File::create(filename)?)),
//     }
// }
