use crate::doc_src::debug_ast;
use crate::items::LineMarker;
use comrak::nodes::{Ast, AstNode, NodeValue};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Clone)]
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

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("name", &self.name.item)
            .field("ast", &debug_ast(&self.ast))
            .finish()
    }
}
