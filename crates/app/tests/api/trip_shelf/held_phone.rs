use std::borrow::Cow;
use std::cell::Cell;
use std::path::PathBuf;

use dioxus::dioxus_core::Mutation;
use dioxus::prelude::*;

use basho::app::App;
use basho::app::session::kept_file::KeptFile;
use basho::app::session::progress_file::ProgressFile;
use basho::app::session::trip_shelf::{PickedFile, TripInput, TripShelf, TripText};
use guide::words::{Label, Spelling};
use storage_ports::HomeFolder;
use tokio::runtime::{Builder, Runtime};

thread_local! {
    static SHELF: Cell<Option<TripShelf>> = const { Cell::new(None) };
}

#[derive(Props, Clone, PartialEq)]
struct Home {
    folder: HomeFolder,
}

pub struct HeldPhone {
    dom: VirtualDom,
    runtime: Runtime,
    edits: Vec<Mutation>,
}

impl HeldPhone {
    const FRESH_PAGE_LISTENER: &str = "touchstart";
    const SAMPLE_YEAR: &str = "2027-";
    const LATER_YEAR: &str = "2090-";
    const SAMPLE: &str = include_str!("../../../../../data/trips/sample.toml");

    pub fn sample() -> PickedFile {
        Self::picked("sample.toml", Some(Self::SAMPLE))
    }

    pub fn later_trip() -> PickedFile {
        let later = Self::SAMPLE.replace(Self::SAMPLE_YEAR, Self::LATER_YEAR);
        Self::picked("later.toml", Some(&later))
    }

    pub fn picked(file: &str, text: Option<&str>) -> PickedFile {
        PickedFile::new(
            Spelling::new(file.to_owned()),
            text.map(|text| TripText::new(Cow::Owned(text.to_owned()))),
        )
    }

    pub fn on_the_sample(home: &HomeFolder) -> HeldPhone {
        home.replace(&home.joined(TripInput::ON_THE_PHONE), &Self::SAMPLE)
            .expect("a trip file");
        Self::opened(home)
    }

    pub fn progress_kept_under(home: &HomeFolder) -> PathBuf {
        let path: PathBuf = KeptFile::under(home, Label::new(ProgressFile::NAME))
            .as_ref()
            .to_path_buf();
        home.replace(&path, &"kept").expect("a progress file");
        path
    }

    pub fn opened(home: &HomeFolder) -> HeldPhone {
        let runtime = Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("the test clock starts");
        let _clock = runtime.enter();
        let mut dom = VirtualDom::new_with_props(
            Self::render,
            Home {
                folder: home.clone(),
            },
        );
        dom.rebuild_in_place();
        HeldPhone {
            dom,
            runtime,
            edits: Vec::new(),
        }
    }

    pub fn offer(&mut self, picked: &PickedFile) {
        let _clock = self.runtime.enter();
        self.dom.in_scope(ScopeId::ROOT, || {
            SHELF.get().expect("the phone is on").offer(picked);
        });
        self.edits = self.dom.render_immediate_to_vec().edits;
    }

    pub fn page_came_up_afresh(&self) -> Option<&Mutation> {
        self.edits.iter().find(|edit| {
            matches!(edit, Mutation::NewEventListener { name, .. } if name == Self::FRESH_PAGE_LISTENER)
        })
    }

    pub fn page(&self) -> String {
        dioxus_ssr::render(&self.dom)
    }

    pub fn refusal(&self) -> Option<String> {
        self.dom.in_scope(ScopeId::ROOT, || {
            SHELF
                .get()
                .and_then(|shelf| shelf.refusal())
                .map(|refusal| refusal.to_string())
        })
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "dioxus hands root props by value"
    )]
    fn render(home: Home) -> Element {
        let shelf = TripShelf::open(None, Some(home.folder.clone()), App::trip_page);
        SHELF.set(Some(shelf));
        shelf.view()
    }
}
