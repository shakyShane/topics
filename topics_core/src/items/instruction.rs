use crate::items::LineMarker;
use comrak::nodes::{Ast, AstNode, NodeValue};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: LineMarker<String>,
    pub ast: Vec<Ast>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ast: vec![],
            name: LineMarker::new(String::new(), None),
        }
    }
}

impl Instruction {
    pub fn set_line_start(&mut self, line_start: usize) {
        self.name.set_line_start(line_start)
    }
}
