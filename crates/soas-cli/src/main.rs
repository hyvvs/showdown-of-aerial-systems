use std::{ffi::OsStr, path::Path, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 4 || args[0] != OsStr::new("run") || args[2] != OsStr::new("--out") {
        eprintln!("Usage: soas-cli run <scenario.json> --out <run.csv>");
        return ExitCode::FAILURE;
    }
    match soas_cli::run_files(Path::new(&args[1]), Path::new(&args[3])) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
