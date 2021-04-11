use crate::doc_src::debug_ast;
use crate::items::LineMarker;
use comrak::nodes::{Ast, AstNode, NodeValue};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: LineMarker<String>,
    pub ast_start: Vec<usize>,
    pub ast_len: usize,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ast_start: vec![],
            ast_len: 0,
            name: LineMarker::new(String::new(), None),
        }
    }
}

// impl fmt::Debug for Instruction {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         f.debug_struct("Instruction")
//             .field("name", &self.name.item)
//             .field("ast_start", &self.ast_start)
//             .field("ast_len", &self.ast_len)
//             .finish()
//     }
// }
