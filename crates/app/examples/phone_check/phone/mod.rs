mod devtools;
mod ports;
mod stopwatch;
mod unfinished;
mod violation;

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use serde_json::{Value, json};

use basho::app::session::trip_shelf::TripShelf;

use crate::ui_check::{Drawing, Handset, Harness, Stage};
use devtools::DevTools;
use layout::Text;
use layout::scale::Token;
pub use ports::{Chrome, MapServer};
pub use stopwatch::Stopwatch;
use unfinished::Unfinished;
use violation::Violation;

pub struct Phone {
    tools: DevTools,
    endpoint: String,
}

impl Phone {
    const STYLE: &str = include_str!("../../../assets/main.css");
    const CHECK: &str = include_str!("../assets/check.js");
    const CHECK_NAME: &str = "bashoCheck";
    const TRIP_FILE_CHECK: &str = include_str!("../assets/trip_file_check.js");
    const MAP_CHECK: &str = include_str!("../assets/map_check.js");
    const MAP_WAIT: Duration = Duration::from_millis(100);
    const MAP_WAITS: usize = 200;
    const MAP_SCALE: f64 = 1.0;
    const MAP_DRAWS: [&str; 4] = ["road_major", "road_street", "water", "goal"];
    const TRIP_FILE_WANTED: &str = r#"{"left":0,"offers":[{"file":"small.toml","text":"[trip]"},{"file":"photo.heic","text":null}],"taken":0}"#;
    const KEPT: &str = "../../target/ui-check";
    const KEEP_ALL: &str = "UI_CHECK_KEEP_ALL";
    const LOADED: &str = "() => 'loaded'";
    const STILL: &str = "*, *::before, *::after { transition: none !important; }";
    const POOL_SIZE: usize = 6;
    const LOAD_TRIES_LEN: usize = 3;

    pub fn open_on(endpoint: &str) -> Phone {
        Self::prepared(DevTools::connect(endpoint).attached(), endpoint)
            .expect("Chrome takes the 430×932 screen, the empty page and the check script")
    }

    pub fn prepared(mut tools: DevTools, endpoint: &str) -> Option<Phone> {
        tools.emulated(&json!({
            "width": Handset::WIDTH.pixels(),
            "height": Handset::HEIGHT.pixels(),
            "deviceScaleFactor": Handset::SCALE.pixels(),
            "mobile": true,
        }))?;
        let shell = format!(
            "document.open(); document.write({}); document.close(); window.{} = {}; `${{innerWidth}}x${{innerHeight}}`",
            serde_json::to_string(&Self::document("")).expect("the page serializes"),
            Self::CHECK_NAME,
            Self::CHECK
        );
        let screen = format!("{}x{}", Handset::WIDTH.pixels(), Handset::HEIGHT.pixels());
        tools
            .evaluated(&shell)
            .filter(|shown| *shown == screen)
            .map(|_| Phone {
                tools,
                endpoint: endpoint.to_owned(),
            })
    }

    pub fn inspect_stages(endpoint: &str, stages: &[Stage], only: Option<&str>) -> Vec<String> {
        let chosen: Arc<Vec<Stage>> = Arc::new(
            stages
                .iter()
                .filter(|stage| only.is_none_or(|only| stage.label().contains(only)))
                .cloned()
                .collect(),
        );
        let queue = Arc::new(AtomicUsize::new(0));
        let running: Vec<_> = (0..Self::POOL_SIZE)
            .map(|_| {
                let endpoint = endpoint.to_owned();
                let chosen = Arc::clone(&chosen);
                let queue = Arc::clone(&queue);
                std::thread::spawn(move || Phone::open_on(&endpoint).inspect_queue(&chosen, &queue))
            })
            .collect();
        running
            .into_iter()
            .flat_map(|worker| worker.join().expect("a check worker finishes"))
            .collect()
    }

    pub fn inspect_queue(mut self, chosen: &[Stage], queue: &AtomicUsize) -> Vec<String> {
        std::iter::from_fn(|| chosen.get(queue.fetch_add(1, Ordering::Relaxed)))
            .flat_map(|stage| self.inspect(stage))
            .collect()
    }

    pub fn inspect(&mut self, stage: &Stage) -> Vec<String> {
        let stage = stage.clone().drawn_with(Drawing::BuiltIn);
        let body = Harness::page(&stage);
        match self.loaded(&body, Self::LOADED) {
            Ok(_) => {
                let found = self.checked(&stage.required());
                Self::reported(&stage, &body, &found)
            }
            Err(unloaded) => Self::reported(&stage, &body, &[unloaded]),
        }
    }

    pub fn loaded(&mut self, body: &str, then: &str) -> Result<String, Violation> {
        let script = format!(
            "(() => {{ document.body.innerHTML = {}; window.scrollTo(0, 0); return JSON.stringify(({then})()); }})()",
            serde_json::to_string(body).expect("the page body serializes")
        );
        (0..Self::LOAD_TRIES_LEN)
            .find_map(|try_index| {
                (try_index > 0).then(|| self.reopened());
                self.tools.evaluated(&script)
            })
            .ok_or_else(|| Unfinished::Unloaded.violation())
    }

    pub fn checked(&mut self, required: &[&str]) -> Vec<Violation> {
        let script = format!(
            "JSON.stringify(window.{}({}, {}))",
            Self::CHECK_NAME,
            serde_json::to_string(required).expect("selectors serialize"),
            Text::Least.design().pixels()
        );
        self.tools
            .evaluated(&script)
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_else(|| vec![Unfinished::Unchecked.violation()])
    }

    fn reopened(&mut self) {
        eprintln!("{self}: the page did not load; loading it again in a new tab");
        let endpoint = self.endpoint.clone();
        DevTools::fresh_on(&endpoint)
            .and_then(|tools| Self::prepared(tools, &endpoint))
            .into_iter()
            .for_each(|phone| *self = phone);
    }

    pub fn reported(stage: &Stage, body: &str, found: &[Violation]) -> Vec<String> {
        let kept = (!found.is_empty() || std::env::var_os(Self::KEEP_ALL).is_some())
            .then(|| Self::keep(stage.label()));
        kept.iter()
            .try_for_each(|kept| fs::write(kept, Self::document(body)))
            .expect("the failing page can be kept");
        found
            .iter()
            .map(|problem| {
                format!(
                    "{stage}: {problem}\n    open {}",
                    kept.as_ref()
                        .map_or_else(String::new, |kept| kept.display().to_string())
                )
            })
            .collect()
    }

    fn keep(label: &str) -> PathBuf {
        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(Self::KEPT);
        fs::create_dir_all(&directory).expect("the kept-pages folder can be made");
        let name: String = label
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    '-'
                }
            })
            .collect();
        directory.join(format!("{name}.html"))
    }

    pub fn document(body: &str) -> String {
        format!(
            "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1, viewport-fit=cover\"><style>{}</style><style>{}</style><style>{}</style></head><body>{body}</body></html>",
            Self::STYLE,
            Handset::SAFE_AREA,
            Self::STILL
        )
    }

    pub fn trip_file_problem(&mut self, stage: &Stage) -> Option<String> {
        let script = format!(
            "({})({})",
            Self::TRIP_FILE_CHECK,
            serde_json::to_string(TripShelf::LISTENER).expect("the listener serializes")
        );
        self.loaded(&Harness::page(stage), Self::LOADED).ok()?;
        let seen = self.tools.awaited(&script);
        let wanted = serde_json::from_str::<Value>(Self::TRIP_FILE_WANTED).ok();
        let read = seen
            .as_deref()
            .and_then(|seen| serde_json::from_str::<Value>(seen).ok());
        (read != wanted).then(|| format!("the trip file listener gave {seen:?}"))
    }

    pub fn inspect_body(&mut self, body: &str, required: &[&str]) -> Vec<Violation> {
        self.loaded(body, Self::LOADED)
            .map_or_else(|unloaded| vec![unloaded], |_| self.checked(required))
    }
}

impl Phone {
    pub fn map_problems(mut self) -> Vec<String> {
        let server = MapServer::started();
        self.tools.replied("Page.bringToFront", &json!({}));
        self.tools.emulated(&json!({
            "width": Handset::WIDTH.pixels(),
            "height": Handset::HEIGHT.pixels(),
            "deviceScaleFactor": Self::MAP_SCALE,
            "mobile": true,
        }));
        self.tools
            .replied("Page.navigate", &json!({ "url": server.page() }));
        let script = format!("({})()", Self::MAP_CHECK);
        let drawn = (0..Self::MAP_WAITS).find_map(|_| {
            std::thread::sleep(Self::MAP_WAIT);
            self.tools
                .evaluated(&script)
                .and_then(|json| serde_json::from_str::<Value>(&json).ok())
                .filter(|report| {
                    Self::MAP_DRAWS
                        .iter()
                        .all(|layer| report["shown"].to_string().contains(layer))
                        || report["failed"] == json!(true)
                })
        });
        drawn.map_or_else(
            || vec!["the offline map never drew its roads, water and goal".to_owned()],
            |report| Self::map_faults(&report),
        )
    }

    fn map_faults(report: &Value) -> Vec<String> {
        let shown = report["shown"].to_string();
        Self::MAP_DRAWS
            .iter()
            .filter(|layer| !shown.contains(*layer))
            .map(|layer| format!("the offline map does not draw {layer}: {shown}"))
            .chain(
                report["away"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|name| format!("the map asked the network for {name}")),
            )
            .collect()
    }
}

impl Phone {
    const PROCESS_INFO: &str = "SystemInfo.getProcessInfo";
    const GPU_INFO: &str = "SystemInfo.getInfo";

    pub fn chrome_usage(endpoint: &str) -> String {
        let mut tools = DevTools::connect(endpoint);
        let processes = tools
            .replied(Self::PROCESS_INFO, &json!({}))
            .and_then(|reply| serde_json::from_str::<Value>(&reply).ok())
            .unwrap_or_default();
        let gpu = tools
            .replied(Self::GPU_INFO, &json!({}))
            .and_then(|reply| serde_json::from_str::<Value>(&reply).ok())
            .unwrap_or_default();
        let cpu = processes["processInfo"]
            .as_array()
            .into_iter()
            .flatten()
            .fold(BTreeMap::<String, f64>::new(), |mut spent, process| {
                *spent
                    .entry(process["type"].as_str().unwrap_or_default().to_owned())
                    .or_default() += process["cpuTime"].as_f64().unwrap_or_default();
                spent
            });
        format!(
            "chrome cpu {cpu:.2?}, webgl {}, gpu {}",
            gpu["gpu"]["featureStatus"]["webgl"], gpu["gpu"]["devices"][0]["deviceString"]
        )
    }
}

impl Drop for Phone {
    fn drop(&mut self) {
        self.tools.close();
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {} px",
            self.tools,
            Handset::WIDTH.pixels()
        )
    }
}
