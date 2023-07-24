use std::io;

use clap::Parser;

// A fabulous AQL formatter.
#[derive(Debug, Parser)]
#[clap(name = "aqlfmt", term_width = 80)]

struct Args {
    // Files or directories.
    #[clap(multiple_values = true)]
    include: Vec<String>,

    // Files or directories to exclude from formatting.
    #[clap(long, short, multiple_occurrences = true)]
    exclude: Vec<String>,

    // Check if the targets has been already formatted.
    #[clap(long, short)]
    check: bool,
}

pub fn main() -> io::Result<()> {
    let args = Args::parse();

    let to_include: Vec<&str> = args
        .include
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();

    let formatted_paths: Vec<String> = match &to_include[..] {
        &[] | &["-"] => {
            vec![]
        }
        include => crate::find::aql_files(include, &args.exclude),
    };

    println!("paths: {}", formatted_paths.join(", "));

    std::process::exit(0)
}
