use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct LineMarker<T: Debug + Clone> {
    pub line_start: Option<usize>,
    pub item: T,
}

impl<T: Debug + Clone> Deref for LineMarker<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: Debug + Clone> LineMarker<T> {
    pub fn new(item: impl Into<T>, line_start: Option<usize>) -> Self {
        Self {
            line_start,
            item: item.into(),
        }
    }
    pub fn set_line_start(&mut self, line_start: usize) {
        self.line_start = Some(line_start)
    }
}
