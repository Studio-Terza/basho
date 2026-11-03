use guide::BufferState;
use guide::DeadlineKind;
use guide::classed::Classed;
use guide::pressing::Pressing;
use guide::readout::Emphasis;

use crate::app::visit::Screen;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Look {
    #[strum(serialize = "screen fit")]
    ScreenFit,
    #[strum(serialize = "screen")]
    ScreenPlain,
    #[strum(serialize = "tab")]
    Tab,
    #[strum(serialize = "tab active")]
    TabActive,
    #[strum(serialize = "menu-day")]
    MenuDay,
    #[strum(serialize = "menu-day current")]
    MenuDayCurrent,
    #[strum(serialize = "day-back")]
    DayBack,
    #[strum(serialize = "day-next")]
    DayNext,
    #[strum(serialize = "")]
    StepPlain,
    #[strum(serialize = " focused")]
    StepFocused,
    #[strum(serialize = "if now")]
    IfNow,
    #[strum(serialize = "if")]
    IfLater,
    #[strum(serialize = "ride")]
    Ride,
    #[strum(serialize = "tour")]
    Tour,
    #[strum(serialize = "cutoff")]
    Cutoff,
    #[strum(serialize = "check-in")]
    CheckIn,
    #[strum(serialize = "comfortable")]
    Comfortable,
    #[strum(serialize = "tight")]
    Tight,
    #[strum(serialize = "short")]
    Short,
    #[strum(serialize = "waiting")]
    Waiting,
    #[strum(serialize = "on-time")]
    OnTime,
    #[strum(serialize = "late")]
    Late,
}

impl Look {
    pub fn screen(screen: &Screen) -> Look {
        if *screen == Screen::Map {
            Look::ScreenFit
        } else {
            Look::ScreenPlain
        }
    }

    pub fn tab(chosen: &Screen, shown: &Screen) -> Look {
        if chosen.tab() == shown.tab() {
            Look::TabActive
        } else {
            Look::Tab
        }
    }

    pub fn menu_day<Today>(current: Option<Today>) -> Look {
        current.map_or(Look::MenuDay, |_| Look::MenuDayCurrent)
    }

    pub fn step_focus(focused: Option<()>) -> Look {
        focused.map_or(Look::StepPlain, |()| Look::StepFocused)
    }

    pub fn pressing(pressing: Pressing) -> Look {
        match pressing {
            Pressing::Now => Look::IfNow,
            Pressing::Later => Look::IfLater,
        }
    }

    pub fn deadline(kind: DeadlineKind) -> Look {
        match kind {
            DeadlineKind::Ride => Look::Ride,
            DeadlineKind::Tour => Look::Tour,
            DeadlineKind::Closes | DeadlineKind::Entry | DeadlineKind::LastOrder => Look::Cutoff,
            DeadlineKind::CheckIn => Look::CheckIn,
        }
    }

    pub fn buffer(state: BufferState) -> Look {
        match state {
            BufferState::Comfortable => Look::Comfortable,
            BufferState::Tight => Look::Tight,
            BufferState::Short => Look::Short,
            BufferState::Waiting => Look::Waiting,
        }
    }

    pub fn time_check(emphasis: Emphasis) -> Look {
        match emphasis {
            Emphasis::Inverted => Look::Late,
            Emphasis::Plain => Look::OnTime,
        }
    }
}

impl Classed for Look {}
