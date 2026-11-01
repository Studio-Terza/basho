mod engine_event;
mod engine_files;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod tile_server;

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use map_plan::{MapSent, MapUpdate};

use crate::app::screen::Screen;
use crate::app::session::Session;
use engine_event::EngineEvent;
use engine_files::EngineFiles;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use tile_server::TileServer;

pub type MapScene = fn(&Session, &mut MapSent) -> Option<MapUpdate>;

#[derive(Clone)]
pub struct MapFeed {
    engine: document::Eval,
    sent: Rc<RefCell<MapSent>>,
    scene: MapScene,
}

impl MapFeed {
    const ENGINE: &str = include_str!("../../../assets/map_engine.js");

    pub fn start(session: &Session, scene: MapScene) {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        TileServer::serve();
        let hearing = session.clone();
        let feed = use_hook(|| {
            let engine = document::eval(Self::ENGINE);
            engine.send(EngineFiles).unwrap_or_default();
            Self::listen(engine, hearing);
            MapFeed {
                engine,
                sent: Rc::new(RefCell::new(MapSent::default())),
                scene,
            }
        });
        let followed = session.clone();
        use_effect(move || feed.follow(&followed));
    }

    fn listen(mut engine: document::Eval, session: Session) {
        spawn(async move {
            let heard = engine
                .recv::<EngineEvent>()
                .await
                .map(|event| event.applied(&session));
            heard.is_ok().then(|| Self::listen(engine, session));
        });
    }

    fn follow(&self, session: &Session) {
        let mode = session.map_mode();
        let drawn = (session.screen() == Screen::Map)
            .then(|| (self.scene)(session, &mut self.sent.borrow_mut()))
            .flatten();
        let moved = self.sent.borrow_mut().mode_update(mode);
        drawn
            .into_iter()
            .chain(moved)
            .try_for_each(|update| self.engine.send(update))
            .unwrap_or_default();
    }
}
