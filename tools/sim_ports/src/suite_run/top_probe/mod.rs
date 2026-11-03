mod process_row;

use std::fmt;
use std::process::{Child, Command, Stdio};

use crate::phase_length::PhaseLength;
use sim_suite::TopSample;

use crate::device::Udid;
use process_row::ProcessRow;

pub struct TopProbe {
    child: Option<Child>,
}

impl TopProbe {
    const LISTING: [&str; 2] = ["-axo", "pid=,ppid=,command="];
    const WATCHED: [&str; 2] = ["Basho.app", "com.apple.WebKit."];
    const HOST: &str = "launchd_sim";

    fn rows() -> Vec<ProcessRow> {
        Command::new("ps")
            .args(Self::LISTING)
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
            .unwrap_or_default()
            .lines()
            .filter_map(|row| row.parse().ok())
            .collect()
    }

    fn pids(udid: &Udid) -> Vec<String> {
        let rows = Self::rows();
        let device = udid.to_string();
        let host = rows.iter().find(|row| {
            row.runs(Self::HOST)
                .and_then(|row| row.runs(&device))
                .is_some()
        });
        rows.iter()
            .filter(|row| host.is_some_and(|host| row.child_of(host)))
            .filter(|row| Self::WATCHED.iter().any(|name| row.runs(name).is_some()))
            .map(|row| row.pid().to_owned())
            .collect()
    }

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    #[must_use]
    pub fn started(udid: &Udid, length: PhaseLength) -> TopProbe {
        let mut top = Command::new("top");
        top.args(["-l", "2", "-s"])
            .arg(length.to_string())
            .args(["-stats", "pid,cpu,power"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        Self::pids(udid).iter().for_each(|pid| {
            top.args(["-pid", pid]);
        });
        TopProbe {
            child: top.spawn().ok(),
        }
    }

    #[must_use]
    pub fn finished(self) -> TopSample {
        self.child
            .and_then(|child| child.wait_with_output().ok())
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
            .and_then(|printed| printed.parse().ok())
            .unwrap_or_default()
    }
}

impl fmt::Display for TopProbe {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(Self::HOST)
    }
}
