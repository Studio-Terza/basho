mod picked_file;
mod shown_trip;
mod trip_input;
mod trip_picker;
mod trip_refusal;
mod trip_text;

use std::ffi::OsString;
use std::rc::Rc;

use dioxus::prelude::*;
use guide::Trip;
use storage_ports::HomeFolder;

use crate::app::session::Session;

pub use picked_file::PickedFile;
use shown_trip::ShownTrip;
pub use trip_input::TripInput;
pub use trip_refusal::TripRefusal;
pub use trip_text::TripText;

pub type TripPage = fn(&Trip, TripShelf) -> Element;

#[derive(Clone, Copy, PartialEq)]
pub struct TripShelf {
    home: Signal<Option<HomeFolder>>,
    shown: Signal<ShownTrip>,
    refusal: Signal<Option<Rc<TripRefusal>>>,
    page: Signal<Option<TripPage>>,
}

impl TripShelf {
    pub const LISTENER: &str = "window.addEventListener('click', (event) => { if (event.target.closest && event.target.closest('.trip-file')) { event.stopPropagation(); } }, true); document.addEventListener('change', async (event) => { const input = event.target; const chosen = input.matches && input.matches('.trip-file') && input.files[0]; if (!chosen) { return; } const text = await chosen.arrayBuffer().then((bytes) => new TextDecoder('utf-8', { fatal: true }).decode(bytes)).catch(() => null); input.value = ''; dioxus.send({ file: chosen.name, text }); }); await new Promise(() => {});";

    pub fn open(named: Option<OsString>, home: Option<HomeFolder>, page: TripPage) -> TripShelf {
        TripShelf {
            shown: use_signal(|| ShownTrip::first(TripInput::read(named, home.as_ref()))),
            home: use_signal(|| home),
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
    pub fn home(&self) -> Option<HomeFolder> {
        self.home.peek().clone()
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
        picked.keep_under(self.home().as_ref())?;
        Session::forget_trip(self.home().as_ref());
        let mut shown = self.shown;
        shown.set(
            picked
                .text()
                .map_or_else(ShownTrip::default, |text| before.next(text, trip)),
        );
        Ok(())
    }
}
