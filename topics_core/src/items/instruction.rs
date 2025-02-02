use std::fmt::Debug;

use crate::doc_src::ast_range::AstRange;

use crate::items::LineMarker;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, serde::Serialize, TypeScriptify)]
pub struct Instruction {
    pub name: LineMarker<String>,
    #[serde(skip)]
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
