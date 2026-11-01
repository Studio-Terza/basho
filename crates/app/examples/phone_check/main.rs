mod phone;
#[path = "../../tests/api/ui_check/mod.rs"]
mod ui_check;

use std::process::ExitCode;

use phone::{Phone, Stopwatch};
use ui_check::{Drawing, Stage};

const BROKEN_PAGE: &str = r#"<nav class="tabs"><button class="tab active">Map</button></nav><div style="width:900px">A row wider than the phone</div><p style="position:absolute;top:40px;margin:0">First line</p><p style="position:absolute;top:44px;margin:0">Second line</p><p class="time-fact" style="width:12px;white-space:normal">10:41</p><p><ruby>字<rt style="font-size:6px">じ</rt></ruby></p>"#;
const MISSING: [&str; 1] = ["section.never-rendered"];
const ONLY: &str = "UI_CHECK_ONLY";
const TRIP_PICKER: &str = "trip picker";
const CAUGHT: [&str; 6] = [
    "page-scroll",
    "overflow",
    "overlap",
    "missing",
    "wrapped-time",
    "small-reading",
];

struct PhoneCheck;

impl PhoneCheck {
    fn missed_on_a_broken_page() -> Vec<String> {
        let problems = Stopwatch::timed("broken page", |endpoint| {
            Phone::open_on(endpoint)
                .inspect_body(BROKEN_PAGE, &MISSING)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        });
        CAUGHT
            .into_iter()
            .filter(|kind| !problems.contains(kind))
            .map(|kind| format!("not caught on the broken page: {kind}\n{problems}"))
            .collect()
    }

    fn trip_file_problems() -> Vec<String> {
        let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
        Stopwatch::timed("trip-file check", |endpoint| {
            let mut phone = Phone::open_on(endpoint);
            Stage::all(&itinerary, &wiki)
                .iter()
                .filter(|stage| stage.label().ends_with(TRIP_PICKER))
                .filter_map(|stage| phone.trip_file_problem(stage))
                .collect()
        })
    }

    fn offline_map_problems() -> Vec<String> {
        Stopwatch::timed("map check", |endpoint| {
            Phone::open_on(endpoint).map_problems()
        })
    }

    fn every_screen_problem() -> Vec<String> {
        let (itinerary, wiki) = (Drawing::BuiltIn.itinerary(), Drawing::BuiltIn.wiki());
        let only = std::env::var(ONLY).ok();
        let stages = Stage::all(&itinerary, &wiki);
        println!("phone check: {} stages", stages.len());
        Stopwatch::timed("stages", |endpoint| {
            Phone::inspect_stages(endpoint, &stages, only.as_deref())
        })
    }
}

fn main() -> ExitCode {
    println!(
        "phone check: {} cores",
        std::thread::available_parallelism().map_or(0, usize::from)
    );
    let checks: [fn() -> Vec<String>; 4] = [
        PhoneCheck::offline_map_problems,
        PhoneCheck::every_screen_problem,
        PhoneCheck::trip_file_problems,
        PhoneCheck::missed_on_a_broken_page,
    ];
    let problems: Vec<String> = std::thread::scope(|scope| {
        checks
            .map(|check| scope.spawn(check))
            .into_iter()
            .flat_map(|running| running.join().expect("a check finishes"))
            .collect()
    });
    if problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        eprintln!(
            "{} layout problems:\n{}",
            problems.len(),
            problems.join("\n")
        );
        ExitCode::FAILURE
    }
}
