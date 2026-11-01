mod class_name;

use class_name::ClassName;

pub trait Classed: Copy + Into<ClassName> {
    fn class(self) -> ClassName {
        self.into()
    }
}
