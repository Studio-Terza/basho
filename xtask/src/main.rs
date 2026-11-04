use std::process::ExitCode;

use console_ports::Process;
use xtask::Command;

fn main() -> ExitCode {
    Process::answer(Command::answer)
}
