use crate::doc::DocResult;
use crate::items::Item;
use comrak::nodes::Ast;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MdElements {
    pub items: Vec<Item>,
}

impl Deref for MdElements {
    type Target = Vec<Item>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
