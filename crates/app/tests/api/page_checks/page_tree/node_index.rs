#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIndex(usize);

impl NodeIndex {
    pub fn new(index: usize) -> NodeIndex {
        NodeIndex(index)
    }

    pub fn index(self) -> usize {
        self.0
    }
}
