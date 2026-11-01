use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, ChildStderr, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

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
    const CACHE: &str = "../../target/ui-chrome-cache";
    const CACHE_HOME: &str = "XDG_CACHE_HOME";
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
        let started = Instant::now();
        println!(
            "phone check: chrome binary {}",
            Self::resolved(&binary).display()
        );
        let cache = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(Self::CACHE);
        std::fs::create_dir_all(&cache).expect("the font cache folder can be made");
        let mut process = Command::new(binary)
            .args(Self::FLAGS)
            .arg(format!("--user-data-dir={}", profile.display()))
            .env(Self::CACHE_HOME, cache)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the UI checks need Chrome or Chromium: set CHROME to its binary");
        let (found, endpoint) = mpsc::channel();
        let stderr = process.stderr.take();
        std::thread::spawn(move || Self::listened(stderr, &found, started));
        let endpoint = endpoint
            .recv_timeout(Self::PATIENCE)
            .expect("Chrome reports its DevTools endpoint");
        println!("phone check: chrome launch {:.2?}", started.elapsed());
        Chrome {
            endpoint,
            process: Some(process),
        }
    }

    fn listened(stderr: Option<ChildStderr>, found: &mpsc::Sender<String>, started: Instant) {
        stderr
            .map(BufReader::new)
            .into_iter()
            .flat_map(BufRead::lines)
            .map_while(Result::ok)
            .inspect(|line| println!("phone check: chrome {:.2?} {line}", started.elapsed()))
            .filter_map(|line| line.strip_prefix(Self::LISTENING).map(str::to_owned))
            .for_each(|endpoint| found.send(endpoint).unwrap_or_default());
    }
}

impl Chrome {
    fn resolved(binary: &str) -> PathBuf {
        std::env::var_os("PATH")
            .iter()
            .flat_map(std::env::split_paths)
            .map(|folder| folder.join(binary))
            .find(|path| path.is_file())
            .and_then(|path| path.canonicalize().ok())
            .unwrap_or_else(|| PathBuf::from(binary))
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
