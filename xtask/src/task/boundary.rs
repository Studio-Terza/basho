use std::ffi::OsString;
use std::io::{Stderr, Stdout, Write};
use std::process::ExitCode;

use file_ports::{Disk, Files};

use crate::task::Command;
use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::ports::Host;

impl Command {
    pub fn answer(arguments: Vec<OsString>, stdout: &mut Stdout, stderr: &mut Stderr) -> ExitCode {
        let mut host = Host::new(&Disk, stdout);
        arguments
            .into_iter()
            .map(|argument| argument.into_string().map_err(Failure::NotText))
            .collect::<Result<Vec<String>, Failure>>()
            .and_then(|arguments| Self::named(&arguments, &mut host))
            .map_or_else(
                |failure| {
                    writeln!(stderr, "{failure}").map_or(ExitCode::FAILURE, |()| ExitCode::FAILURE)
                },
                |()| ExitCode::SUCCESS,
            )
    }

    fn named(
        arguments: &[String],
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        arguments
            .first()
            .ok_or_else(Command::usage)
            .and_then(|argument| argument.parse::<Command>())
            .and_then(|command| command.run(arguments.iter().skip(1).cloned().map(Flag::new), host))
    }
}
