#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wakes(u32);

impl Wakes {
    pub const LISTENER: &str = "document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'visible') { dioxus.send(true); } }); window.addEventListener('pageshow', () => dioxus.send(true)); await new Promise(() => {});";

    #[cfg_attr(
        not(target_os = "ios"),
        expect(dead_code, reason = "only the iOS location feed restarts on waking")
    )]
    pub fn ever(self) -> bool {
        self != Wakes::default()
    }
}

impl Wakes {
    #[must_use]
    pub fn next(self) -> Wakes {
        Wakes(self.0.wrapping_add(1))
    }
}
