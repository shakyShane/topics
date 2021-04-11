use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use comrak::nodes::{Ast, AstNode, NodeValue};

use crate::doc_src::ast_range::AstRange;
use crate::doc_src::debug_ast;
use crate::items::LineMarker;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: LineMarker<String>,
    pub ast_range: AstRange,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ast_range: AstRange::default(),
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
