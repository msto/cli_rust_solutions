use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input text.", required = true)]
    text: Vec<String>,

    #[arg(short = 'n', default_value_t = false, help = "Do not print newline.")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    let ending = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), ending);
}

// Clap v2 (Builder API)
//
// use clap::{App, Arg};
//
// fn main() {
//     let matches = App::new("echor")
//         .version("0.1.0")
//         .author("Matt Stone <matthew.stone12@gmail.com>")
//         .about("Rust echo")
//         .arg(
//             Arg::with_name("text")
//                 .value_name("TEXT")
//                 .help("Input text")
//                 .required(true)
//                 .min_values(1),
//         )
//         .arg(
//             Arg::with_name("omit_newline")
//                 .short("n")
//                 .help("Do not print newline")
//                 .takes_value(false),
//         )
//         .get_matches();

//     let text = matches.values_of_lossy("text").unwrap();
//     let omit_newline = matches.is_present("omit_newline");

//     let ending = if omit_newline { "" } else { "\n" };
//     print!("{}{}", text.join(" "), ending);
// }
