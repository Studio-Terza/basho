mod picked_file;
mod shown_trip;
mod trip_input;
mod trip_picker;
mod trip_refusal;
mod trip_text;

use std::ffi::OsString;
use std::path::PathBuf;
use std::rc::Rc;

use dioxus::prelude::*;
use guide::Trip;

use crate::app::session::Session;

pub use picked_file::PickedFile;
use shown_trip::ShownTrip;
pub use trip_input::TripInput;
pub use trip_refusal::TripRefusal;
pub use trip_text::TripText;

pub type TripPage = fn(&Trip, TripShelf) -> Element;

#[derive(Clone, Copy, PartialEq)]
pub struct TripShelf {
    home: Signal<Option<PathBuf>>,
    shown: Signal<ShownTrip>,
    refusal: Signal<Option<Rc<TripRefusal>>>,
    page: Signal<Option<TripPage>>,
}

impl TripShelf {
    pub const LISTENER: &str = "window.addEventListener('click', (event) => { if (event.target.closest && event.target.closest('.trip-file')) { event.stopPropagation(); } }, true); document.addEventListener('change', async (event) => { const input = event.target; const chosen = input.matches && input.matches('.trip-file') && input.files[0]; if (!chosen) { return; } const text = await chosen.arrayBuffer().then((bytes) => new TextDecoder('utf-8', { fatal: true }).decode(bytes)).catch(() => null); input.value = ''; dioxus.send({ file: chosen.name, text }); }); await new Promise(() => {});";

    pub fn open(named: Option<OsString>, home: Option<OsString>, page: TripPage) -> TripShelf {
        let kept_home = home.clone().map(PathBuf::from);
        TripShelf {
            home: use_signal(|| kept_home),
            shown: use_signal(|| ShownTrip::first(TripInput::read(named, home))),
            refusal: use_signal(|| None),
            page: use_signal(|| Some(page)),
        }
    }

    pub fn staged(refusal: Option<TripRefusal>) -> TripShelf {
        TripShelf {
            home: use_signal(|| None),
            shown: use_signal(ShownTrip::default),
            refusal: use_signal(|| refusal.map(Rc::new)),
            page: use_signal(|| None),
        }
    }

    pub fn listen(self, mut listener: document::Eval) {
        spawn(async move {
            let heard = listener
                .recv::<PickedFile>()
                .await
                .map(|picked| self.offer(&picked));
            heard.is_ok().then(|| self.listen(listener));
        });
    }

    pub fn view(self) -> Element {
        self.shown.read().view(self)
    }

    #[must_use]
    pub fn page(&self) -> Option<TripPage> {
        *self.page.peek()
    }

    #[must_use]
    pub fn trip(&self) -> Option<Rc<Trip>> {
        self.shown.peek().trip()
    }

    #[must_use]
    pub fn refusal(&self) -> Option<Rc<TripRefusal>> {
        self.refusal.read().clone()
    }

    pub fn offer(self, picked: &PickedFile) {
        let mut refusal = self.refusal;
        refusal.set(self.taken(picked).err().map(Rc::new));
    }

    fn taken(self, picked: &PickedFile) -> Result<(), TripRefusal> {
        let before = self.shown.peek().clone();
        picked
            .text()
            .filter(|text| before.source() == Some(*text))
            .map_or_else(|| self.switched(picked, &before), |_| Ok(()))
    }

    fn switched(self, picked: &PickedFile, before: &ShownTrip) -> Result<(), TripRefusal> {
        let trip = picked.trip()?;
        picked.keep_under(self.home.peek().as_deref())?;
        Session::forget_trip(self.home.peek().as_deref());
        let mut shown = self.shown;
        shown.set(
            picked
                .text()
                .map_or_else(ShownTrip::default, |text| before.next(text, trip)),
        );
        Ok(())
    }
}
