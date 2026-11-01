use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStderr, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::Duration;

static LAUNCHED: AtomicUsize = AtomicUsize::new(0);

pub struct Chrome {
    endpoint: String,
    process: Option<Child>,
}

impl Chrome {
    const ENDPOINT: &str = "BASHO_CHROME_ENDPOINT";
    const BINARY: &str = "CHROME";
    const FALLBACK: &str = "chromium";
    const LISTENING: &str = "DevTools listening on ";
    const PROFILE: &str = "../../target/ui-chrome-own";
    const PATIENCE: Duration = Duration::from_secs(30);
    const FLAGS: [&str; 8] = [
        "--headless",
        "--use-angle=swiftshader",
        "--enable-unsafe-swiftshader",
        "--remote-debugging-port=0",
        "--no-sandbox",
        "--no-first-run",
        "--disable-extensions",
        "--force-color-profile=srgb",
    ];

    pub fn shared() -> Chrome {
        std::env::var(Self::ENDPOINT).map_or_else(
            |_| {
                Self::launched(
                    std::env::var(Self::BINARY).unwrap_or_else(|_| Self::FALLBACK.to_owned()),
                )
            },
            |endpoint| Chrome {
                endpoint,
                process: None,
            },
        )
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn launched(binary: String) -> Chrome {
        let profile = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "{}-{}-{}",
            Self::PROFILE,
            std::process::id(),
            LAUNCHED.fetch_add(1, Ordering::Relaxed)
        ));
        let mut process = Command::new(binary)
            .args(Self::FLAGS)
            .arg(format!("--user-data-dir={}", profile.display()))
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the UI checks need Chrome or Chromium: set CHROME to its binary");
        let (found, endpoint) = mpsc::channel();
        let stderr = process.stderr.take();
        std::thread::spawn(move || Self::listened(stderr, &found));
        let endpoint = endpoint
            .recv_timeout(Self::PATIENCE)
            .expect("Chrome reports its DevTools endpoint");
        Chrome {
            endpoint,
            process: Some(process),
        }
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

impl fmt::Display for Chrome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.endpoint)
    }
}

impl Drop for Chrome {
    fn drop(&mut self) {
        self.process.iter_mut().for_each(|process| {
            process.kill().ok();
            process.wait().ok();
        });
    }
}
