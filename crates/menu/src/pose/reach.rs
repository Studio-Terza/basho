use guide::words::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Live,
    Inert,
}

impl Reach {
    pub fn pointer_events(self) -> Label {
        match self {
            Reach::Live => Label::new("auto"),
            Reach::Inert => Label::new("none"),
        }
    }
}
