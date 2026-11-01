use guide::words::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Live,
    Inert,
}

impl Reach {
    pub fn pointer_events(self) -> Label {
        Label::new(match self {
            Reach::Live => "auto",
            Reach::Inert => "none",
        })
    }
}
