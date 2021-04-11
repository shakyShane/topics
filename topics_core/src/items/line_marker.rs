use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct LineMarker<T>
where
    T: Debug + Clone + Default,
{
    pub line_start: Option<usize>,
    pub item: T,
}

impl<T> Deref for LineMarker<T>
where
    T: Debug + Clone + Default,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> LineMarker<T>
where
    T: Debug + Clone + Default,
{
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

impl PartialEq<String> for LineMarker<String> {
    fn eq(&self, other: &String) -> bool {
        self.item == *other
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
