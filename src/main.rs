pub mod days;
use std::{env, process::ExitCode};

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    args.next().expect("Program path needs to be provided");
    let day = args.next().ok_or_else(|| {
        eprint!("ERROR: no day is provided");
    })?;
    match day.as_str() {
        "day01" => {
            days::day01::solve();
        }
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
