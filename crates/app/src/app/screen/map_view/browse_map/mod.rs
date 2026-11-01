mod browse_line;
mod live_line;
use dioxus::prelude::*;
use guide::classed::Classed;

use super::day_map::DayMapView;
use super::picked_line::PickedLine;
use super::stepper::Stepper;
use crate::app::screen::map_view::narration::Narration;
use crate::app::screen::step_title::StepTitle;
use crate::app::session::{Session, SessionEvent};
use crate::app::step::{Spot, Step, Toward};
use browse_line::BrowseLine;
use live_line::LiveLine;

pub struct BrowseMap<'session> {
    session: &'session Session,
    spot: Spot,
}

impl<'session> BrowseMap<'session> {
    pub fn new(session: &'session Session, spot: Spot) -> Self {
        BrowseMap { session, spot }
    }

    pub fn render_found(self) -> Option<Element> {
        let language = self.session.language();
        let day = self.session.itinerary().day_number(self.spot.day())?;
        let steps = Step::of_day(day);
        let step = steps
            .iter()
            .find(|step| step.number() == self.spot.step())?;
        let today = self.session.day_at(self.session.now());
        let walker = today.map_or_else(
            || self.session.guidance(day),
            |today| self.session.guidance(today),
        );
        let heading = BrowseLine::new(self.spot);
        let title = StepTitle::new(step);
        let opener = self.session.clone();
        let home = self.session.clone();
        Some(rsx! {
            div { class: "map-screen browsing",
                {today.map(|_| LiveLine::new(Narration::new(&walker, self.session.language())))}
                section { class: "browse-banner", role: "group", "aria-label": "Browsing",
                    button { class: "browse-title", "aria-label": "{heading} · {title}",
                        onclick: move |_| SessionEvent::ShowFocusInToday.applied(&opener),
                        p { class: "browse-what", "{heading}" }
                        p { class: "step-title", lang: language.class(), {StepTitle::new(step)} }
                    }
                    div { class: "browse-controls",
                        {Stepper::new(self.session).render(Toward::Previous)}
                        button { class: "back-to-now", "aria-label": "Back to now",
                            onclick: move |_| SessionEvent::BackToNow.applied(&home),
                            "Back to now"
                        }
                        {Stepper::new(self.session).render(Toward::Next)}
                    }
                }
                {PickedLine::new(self.session)}
                div { class: "map-wrap", {DayMapView::new(self.session)} }
            }
        })
    }
}
