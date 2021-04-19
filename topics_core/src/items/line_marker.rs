use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Default, Eq, Hash, PartialEq, Clone, serde::Serialize)]
pub struct LineMarker<T>
where
    T: Debug + Default + Eq + Hash + PartialEq + Clone,
{
    pub line_start: Option<u32>,
    pub item: T,
}

impl Debug for LineMarker<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}]: {}", self.line_start, self.item)
    }
}

impl<T> Deref for LineMarker<T>
where
    T: Debug + Default + Eq + Hash + PartialEq + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> LineMarker<T>
where
    T: Debug + Default + Eq + Hash + PartialEq + Clone,
{
    pub fn new(item: impl Into<T>, line_start: Option<u32>) -> Self {
        Self {
            line_start,
            item: item.into(),
        }
    }
    pub fn set_line_start(&mut self, line_start: u32) {
        self.line_start = Some(line_start)
    }
}

impl PartialEq<String> for LineMarker<String> {
    fn eq(&self, other: &String) -> bool {
        self.item == *other
    }
}

impl<T> Display for LineMarker<T>
where
    T: Debug + Default + Eq + Hash + PartialEq + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl FromStr for LineMarker<String> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LineMarker::new(s.to_string(), None))
    }
}

impl From<&str> for LineMarker<String> {
    fn from(input: &str) -> Self {
        LineMarker::new(input.to_string(), None)
    }
}
