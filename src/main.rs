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

    #[clap(
        short,
        long,
        default_value = "1",
        about = "The number of threads to run in parallel"
    )]
    parallel: usize,

    #[clap(
        short,
        long,
        about = "Replace occurences of this with args read from stdin"
    )]
    replace: Option<String>,

    #[clap(
        multiple = true,
        min_values = 1,
        about = "The command to execute against the args"
    )]
    command: Vec<String>,
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
    let args: String = match &opts.arg_file {
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
    let mut orig_command = opts.command.clone();
    let command = orig_command.remove(0);

    let delimited: Vec<String> = args.split(delimiter).map(|s| s.to_string()).collect();
    delimited.par_iter().for_each(|arg| {
        let mut command_args = orig_command.clone();
        match &opts.replace {
            // By default the arg from stdin is added to the end.
            None => {
                command_args.push(arg.to_string());
            }
            // Otherwise we replace occurences of a given replace-str with the
            // string from stdin.
            Some(replace_str) => {
                command_args = command_args
                    .iter()
                    .map(|s| s.replace(replace_str, arg))
                    .collect::<Vec<String>>();
            }
        }

        // When .status() is used, the stdout/stderr are inherited
        Command::new(&command)
            .args(command_args)
            .status()
            .expect("Failed to execute process");
    });
    Ok(())
}
