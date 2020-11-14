use clap::Clap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Vasilios Syrakis <cetanu@gmail.com>")]
struct Opts {
    #[clap(short, long, about = "Read args from a file")]
    arg_file: Option<PathBuf>,

    #[clap(short, long, about = "Split the args by a particular delimiter")]
    delimiter: Option<char>,

    #[clap(min_values = 1, about = "The command to execute against the args")]
    command: String,

    #[clap(
        short,
        long,
        default_value = "1",
        about = "The number of threads to run in parallel"
    )]
    parallel: usize,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(opts.parallel)
        .build_global()
        .unwrap();

    // I think original xargs uses \0 to check for null-terminated strings
    // This means that newlines are retained...
    // I don't think this is good, but I could be convinced otherwise
    let mut delimiter: char = '\n';
    if let Some(d) = opts.delimiter {
        delimiter = d;
    }

    // If args_file is specified, it takes precedence
    let mut buffer = String::new();
    let args: String = match opts.arg_file {
        Some(file) => {
            let file = File::open(&file)?;
            let mut reader = io::BufReader::new(file);
            reader.read_to_string(&mut buffer)?;
            buffer.trim().to_string()
        }
        None => {
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Could not read from stdin");
            buffer.trim().to_string()
        }
    };

    // Grab a mutable copy so we can cut off additional args
    // that may be present in the command.
    let mut command = opts.command.clone();
    // Splitting off the args that came in from stdin.
    // These will be placed before the args provided to zargs
    let mut piped_args: Vec<String> = Vec::new();
    if let Some(idx) = command.find(' ') {
        piped_args.push(command.split_off(idx - 1));
    }

    let delimited: Vec<String> = args.split(delimiter).map(|s| s.to_string()).collect();

    delimited.par_iter().for_each(|arg| {
        // When .status() is used, the stdout/stderr are inherited
        Command::new(&command)
            .args(&piped_args)
            .arg(arg)
            .status()
            .expect("Failed to execute process");
    });
    Ok(())
}
