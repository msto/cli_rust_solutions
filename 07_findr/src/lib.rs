use crate::EntryType::*;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    #[value(name("d"))]
    // #[value(alias("d"))]
    Dir,

    #[value(name("f"))]
    File,

    #[value(name("l"))]
    Link,
}

// impl

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Search paths", id = "PATH", default_value = ".")]
    paths: Vec<String>,

    #[arg(short = 'n', long = "name", help = "Name", id = "NAME", num_args=0..)]
    names: Vec<Regex>,

    #[arg(short = 't', long = "type", help = "Entry type", id = "TYPE", num_args=0..)]
    types: Vec<EntryType>,
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);
    let is_valid_type = |entry: &DirEntry| -> bool {
        args.types.is_empty()
            || args.types.iter().any(|etype| match etype {
                Dir => entry.file_type().is_dir(),
                File => entry.file_type().is_file(),
                Link => entry.file_type().is_symlink(),
            })
    };

    let is_valid_name = |entry: &DirEntry| -> bool {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(is_valid_type)
            .filter(is_valid_name)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        // TODO: why can't I map a print statement over this instead of
        // storing all lines in memory
        // .map(|entry| println!(entry.path().display())?;

        println!("{}", entries.join("\n"));
    }

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
