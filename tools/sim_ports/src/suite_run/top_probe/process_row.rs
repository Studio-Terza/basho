use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessRow {
    pid: String,
    parent: String,
    command: String,
}

impl ProcessRow {
    #[must_use]
    pub fn pid(&self) -> &str {
        &self.pid
    }

    #[must_use]
    pub fn child_of(&self, host: &ProcessRow) -> bool {
        self.parent == host.pid
    }

    #[must_use]
    pub fn runs(&self, name: &str) -> Option<&ProcessRow> {
        self.command.contains(name).then_some(self)
    }
}

impl FromStr for ProcessRow {
    type Err = std::convert::Infallible;

    fn from_str(row: &str) -> Result<ProcessRow, Self::Err> {
        let (pid, rest) = row.trim_start().split_once(' ').unwrap_or_default();
        let (parent, command) = rest.trim_start().split_once(' ').unwrap_or_default();
        Ok(ProcessRow {
            pid: pid.to_owned(),
            parent: parent.to_owned(),
            command: command.to_owned(),
        })
    }
}
