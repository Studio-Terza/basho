use std::env;
use std::process::ExitCode;
use xtask::task::flag::Flag;

use xtask::Command;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = arguments
        .first()
        .ok_or_else(Command::usage)
        .and_then(|argument| argument.parse::<Command>());
    match command.and_then(|command| command.run(arguments.into_iter().skip(1).map(Flag::new))) {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::FAILURE
        }
    }
}
