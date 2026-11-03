use guide::BufferState;
use guide::classed::Classed;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpareLook {
    #[strum(serialize = "spare")]
    Plain,
    #[strum(serialize = "spare inverted")]
    Inverted,
}

impl SpareLook {
    pub fn for_state(state: BufferState) -> SpareLook {
        match state {
            BufferState::Comfortable | BufferState::Waiting => SpareLook::Plain,
            BufferState::Tight | BufferState::Short => SpareLook::Inverted,
        }
    }
}

impl Classed for SpareLook {}
