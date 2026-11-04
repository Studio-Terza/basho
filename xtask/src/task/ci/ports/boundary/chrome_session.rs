use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::task::chromium::Chromium;
use crate::task::failure::Failure;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use guide::words::Label;

#[must_use]
pub struct ChromeSession {
    chrome: Child,
    profile: PathBuf,
}

impl ChromeSession {
    const BINARY: &str = "CHROME";
    const ENDPOINT: &str = "BASHO_CHROME_ENDPOINT";
    const TARGET: &str = "CARGO_TARGET_DIR";
    const PROFILE: &str = "ui-chrome";
    const LISTENING: &str = "DevTools listening on ";
    const PATIENCE: Duration = Duration::from_secs(30);
    const SCHEME: &str = "ws://";
    const NEW_TAB: &str =
        "PUT /json/new?about:blank HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    const TABS: [&str; 6] = ["about:blank"; 6];
    const FLAGS: [&str; 14] = [
        "--headless",
        "--use-angle=swiftshader",
        "--enable-unsafe-swiftshader",
        "--remote-debugging-port=0",
        "--no-sandbox",
        "--no-first-run",
        "--disable-extensions",
        "--disable-background-networking",
        "--disable-renderer-backgrounding",
        "--force-color-profile=srgb",
        "--allow-file-access-from-files",
        "--disable-gpu-compositing",
        "--disable-gpu-vsync",
        "--disable-frame-rate-limit",
    ];

    pub fn start(
        workspace: &Workspace,
        chromium: Option<&Chromium>,
    ) -> Result<Option<ChromeSession>, Failure> {
        chromium
            .map(|chromium| chromium.binary().as_os_str().to_os_string())
            .or_else(|| env::var_os(Self::BINARY))
            .map(|binary| Self::launched(workspace, &binary))
            .transpose()
    }

    fn launched(workspace: &Workspace, binary: &OsStr) -> Result<ChromeSession, Failure> {
        let started = Instant::now();
        let profile = env::var_os(Self::TARGET)
            .map_or_else(|| workspace.root().join("target"), PathBuf::from)
            .join(format!("{}-{}", Self::PROFILE, std::process::id()));
        let io = |error| Failure::Io {
            site: Site::from(Path::new(binary)),
            error,
        };
        let mut chrome = Command::new(binary)
            .args(Self::FLAGS)
            .arg(format!("--user-data-dir={}", profile.display()))
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(io)?;
        let (found, endpoint) = mpsc::channel();
        let stderr = chrome.stderr.take();
        thread::spawn(move || Self::listened(stderr, &found));
        let session = ChromeSession { chrome, profile };
        let endpoint = endpoint
            .recv_timeout(Self::PATIENCE)
            .map_err(|_| Failure::Environment(Label::new(Self::BINARY)))?;
        Self::opened_tabs(&endpoint).map_err(io)?;
        unsafe {
            env::set_var(Self::ENDPOINT, endpoint);
        }
        println!("ci: chrome {:.1?}", started.elapsed());
        Ok(session)
    }

    fn opened_tabs(endpoint: &str) -> io::Result<()> {
        let address = endpoint
            .trim_start_matches(Self::SCHEME)
            .split('/')
            .next()
            .unwrap_or_default();
        let opening: Vec<_> = Self::TABS
            .iter()
            .map(|_| TcpStream::connect(address))
            .map(|stream| thread::spawn(move || Self::answered(stream?)))
            .collect();
        opening.into_iter().try_for_each(|tab| {
            tab.join()
                .unwrap_or_else(|_| Err(io::ErrorKind::Other.into()))
        })
    }

    fn answered(mut stream: TcpStream) -> io::Result<()> {
        stream.write_all(Self::NEW_TAB.as_bytes())?;
        BufReader::new(stream)
            .read_line(&mut String::new())
            .map(drop)
    }

    fn listened(stderr: Option<ChildStderr>, found: &mpsc::Sender<String>) {
        stderr
            .map(BufReader::new)
            .into_iter()
            .flat_map(BufRead::lines)
            .map_while(Result::ok)
            .filter_map(|line| line.strip_prefix(Self::LISTENING).map(str::to_owned))
            .for_each(|endpoint| found.send(endpoint).unwrap_or_default());
    }
}

impl fmt::Display for ChromeSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Chrome #{}", self.chrome.id())
    }
}

impl Drop for ChromeSession {
    fn drop(&mut self) {
        self.chrome.kill().ok();
        self.chrome.wait().ok();
        std::fs::remove_dir_all(&self.profile).ok();
    }
}
