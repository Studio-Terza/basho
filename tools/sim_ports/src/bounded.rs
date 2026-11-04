use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::iter;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};
use tool_text::{ToolName, ToolOutput};

use crate::failure::Failure;

pub struct Bounded {
    command: Command,
    tool: String,
    limit: Duration,
    log: Option<PathBuf>,
}

impl Bounded {
    const KILL: &str = "/bin/kill";
    const SIGNAL: &str = "-KILL";
    const SIMCTL: &str = "simctl";
    const DETAIL_LEN: usize = 12;

    #[must_use]
    pub fn named(tool: &str, limit: Duration) -> Bounded {
        let mut command = Command::new(tool);
        command
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        Bounded {
            command,
            tool: tool.to_owned(),
            limit,
            log: None,
        }
    }

    #[must_use]
    pub fn simctl(limit: Duration) -> Bounded {
        Bounded {
            tool: Self::SIMCTL.to_owned(),
            ..Self::named("xcrun", limit).arg(Self::SIMCTL)
        }
    }

    #[must_use]
    pub fn arg(mut self, argument: impl AsRef<OsStr>) -> Bounded {
        self.command.arg(argument);
        self
    }

    #[must_use]
    pub fn args<Item: AsRef<OsStr>>(
        mut self,
        arguments: impl IntoIterator<Item = Item>,
    ) -> Bounded {
        self.command.args(arguments);
        self
    }

    #[must_use]
    pub fn env(mut self, key: impl AsRef<OsStr>, setting: impl AsRef<OsStr>) -> Bounded {
        self.command.env(key, setting);
        self
    }

    #[must_use]
    pub fn within(mut self, directory: &Path) -> Bounded {
        self.command.current_dir(directory);
        self
    }

    #[must_use]
    pub fn logged_to(self, path: PathBuf) -> Bounded {
        Bounded {
            log: Some(path),
            ..self
        }
    }

    pub fn printed(self) -> Result<String, Failure> {
        self.heard(|_| ())
    }

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    pub fn heard(mut self, hearing: impl FnMut(&str)) -> Result<String, Failure> {
        let mut child = self.command.spawn().map_err(|error| self.failed(&error))?;
        let (sender, lines) = mpsc::channel();
        let readers: Vec<_> = [
            child
                .stdout
                .take()
                .map(|stream| Self::read(stream, sender.clone())),
            child.stderr.take().map(|stream| Self::read(stream, sender)),
        ]
        .into_iter()
        .flatten()
        .collect();
        let deadline = Instant::now() + self.limit;
        let transcript = Self::transcript(&lines, deadline, hearing).map_err(|_| {
            Self::stop(&mut child);
            Failure::TimedOut {
                tool: ToolName::new(self.tool.clone()),
                limit: self.limit,
            }
        })?;
        readers.into_iter().for_each(|reader| {
            reader.join().unwrap_or_default();
        });
        self.log
            .iter()
            .try_for_each(|path| fs::write(path, &transcript))
            .map_err(|error| self.failed(&error))?;
        let status = child.wait().map_err(|error| self.failed(&error))?;
        status
            .success()
            .then_some(transcript.clone())
            .ok_or_else(|| Failure::Tool {
                tool: ToolName::new(self.tool.clone()),
                detail: ToolOutput::new(Self::tail(&transcript)),
            })
    }

    fn transcript(
        lines: &Receiver<String>,
        deadline: Instant,
        mut hearing: impl FnMut(&str),
    ) -> Result<String, RecvTimeoutError> {
        iter::repeat_with(|| lines.recv_timeout(deadline.saturating_duration_since(Instant::now())))
            .take_while(|line| *line != Err(RecvTimeoutError::Disconnected))
            .try_fold(String::new(), |transcript, line| {
                line.map(|line| {
                    hearing(&line);
                    transcript + &line + "\n"
                })
            })
    }

    pub fn tail(transcript: &str) -> String {
        let lines: Vec<&str> = transcript.lines().collect();
        lines[lines.len().saturating_sub(Self::DETAIL_LEN)..].join("\n")
    }

    fn read(stream: impl Read + Send + 'static, lines: Sender<String>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            BufReader::new(stream)
                .lines()
                .map_while(Result::ok)
                .try_for_each(|line| lines.send(line))
                .unwrap_or_default();
        })
    }

    fn stop(child: &mut Child) {
        Command::new(Self::KILL)
            .arg(Self::SIGNAL)
            .arg(format!("-{}", child.id()))
            .status()
            .map(drop)
            .unwrap_or_default();
        child.wait().map(drop).unwrap_or_default();
    }

    fn failed(&self, error: &std::io::Error) -> Failure {
        Failure::Tool {
            tool: ToolName::new(self.tool.clone()),
            detail: ToolOutput::new(error.to_string()),
        }
    }
}

impl fmt::Display for Bounded {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.tool)
    }
}
