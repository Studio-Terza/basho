use guide::clock::TripTime;
use guide::geo::{Bearing, Coordinate, Distance};
use guide::guidance::LegProgress;
use guide::guidance::timeline::Halt;
use layout::scale::Viewport;
use map_plan::{KeyEntry, MapLevel, MapMode, PickedMark};
use report::ReportStore;

use crate::app::visit::Screen;
use std::sync::Arc;

use wiki::Wiki;

use crate::app::history::History;
use crate::app::session::LocationAccess;
use crate::app::session::trip_shelf::TripRefusal;
use crate::app::session::wall_clock::WallClock;
use crate::app::visit::{Focus, Landing};
use menu::Menu;

pub trait Staged {
    fn screen(&self) -> &Screen;
    fn focus(&self) -> Focus;
    fn landing(&self) -> Landing;
    fn menu(&self) -> Menu;
    fn map_level(&self) -> MapLevel;
    fn map_mode(&self) -> MapMode;
    fn key_shown(&self) -> Vec<KeyEntry>;
    fn picked(&self) -> Option<PickedMark>;
    fn viewport(&self) -> Viewport;
    fn position(&self) -> Option<Coordinate>;
    fn spread(&self) -> Option<Distance>;
    fn heading(&self) -> Option<Bearing>;
    fn access(&self) -> LocationAccess;
    fn trail(&self) -> &[Coordinate];
    fn halt(&self) -> Option<Halt>;
    fn progress(&self) -> LegProgress;
    fn now(&self) -> TripTime;
    fn clock(&self) -> WallClock;
    fn reports(&self) -> Option<ReportStore>;
    fn refusal(&self) -> Option<TripRefusal>;
    fn starting_history(&self) -> History;
    fn wiki(&self) -> Arc<Wiki>;
}
