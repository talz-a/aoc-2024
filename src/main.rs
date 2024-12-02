pub mod days;
use std::{env, process::ExitCode};

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    args.next().expect("ERROR: Program path needs to be provided");
    let day = args.next().ok_or_else(|| {
        eprint!("USSAGE: cargo run <day>");
    })?;
    match day.as_str() {
        "day01" => days::day01::solve(),
        "day02" => days::day02::solve(),
        _ => {
            eprint!("ERROR: day not implemented");
            return Err(());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
