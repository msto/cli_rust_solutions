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

impl Args {
    fn set_defaults(&mut self) {
        if self.types.is_empty() {
            self.types = vec![Dir, File, Link];
        }
    }
}

pub fn run(args: Args) -> MyResult<()> {
    // dbg!(args);
    let is_valid_type = |entry: &DirEntry| -> bool {
        args.types.iter().any(|etype| match etype {
            Dir => entry.file_type().is_dir(),
            File => entry.file_type().is_file(),
            Link => entry.file_type().is_symlink(),
        })
    };

    let is_valid_name = |entry: &DirEntry| -> bool {
        if (&args.names).is_empty() {
            return true;
        }

        for re in &args.names {
            // if re.is_match(entry.path().to_str().unwrap()) {
            if re.is_match(entry.file_name().to_str().unwrap()) {
                return true;
            }
        }

        false
    };

    let print = |entry: DirEntry| -> MyResult<()> {
        if is_valid_type(&entry) && is_valid_name(&entry) {
            println!("{}", entry.path().display());
        }

        Ok(())
    };

    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                // Ok(entry) => println!("{}", entry.path().display()),
                Ok(entry) => print(entry)?,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Args> {
    let mut args = Args::parse();
    args.set_defaults();

    Ok(args)
}

fn _open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
