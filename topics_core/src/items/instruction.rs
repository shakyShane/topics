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

fn debug_ast(asts: &[Ast]) -> impl Debug {
    use std::fmt::Write;
    let mut s = String::new();
    struct AstDebug(Vec<Ast>);
    impl Debug for AstDebug {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let items = self
                .0
                .iter()
                .map(|ast| {
                    let name = match ast.value {
                        NodeValue::Document => "Document",
                        NodeValue::FrontMatter(_) => "FrontMatter",
                        NodeValue::BlockQuote => "BlockQuote",
                        NodeValue::List(_) => "List",
                        NodeValue::Item(_) => "Item",
                        NodeValue::DescriptionList => "DescriptionList",
                        NodeValue::DescriptionItem(_) => "DescriptionItem",
                        NodeValue::DescriptionTerm => "DescriptionTerm",
                        NodeValue::DescriptionDetails => "DescriptionDetails",
                        NodeValue::CodeBlock(_) => "CodeBlock",
                        NodeValue::HtmlBlock(_) => "HtmlBlock",
                        NodeValue::Paragraph => "Paragraph",
                        NodeValue::Heading(_) => "Heading",
                        NodeValue::ThematicBreak => "ThematicBreak",
                        NodeValue::FootnoteDefinition(_) => "FootnoteDefinition",
                        NodeValue::Table(_) => "Table",
                        NodeValue::TableRow(_) => "TableRow",
                        NodeValue::TableCell => "TableCell",
                        NodeValue::Text(_) => "Text",
                        NodeValue::TaskItem(_) => "TaskItem",
                        NodeValue::SoftBreak => "SoftBreak",
                        NodeValue::LineBreak => "LineBreak",
                        NodeValue::Code(_) => "Code",
                        NodeValue::HtmlInline(_) => "HtmlInline",
                        NodeValue::Emph => "Emph",
                        NodeValue::Strong => "Strong",
                        NodeValue::Strikethrough => "Strikethrough",
                        NodeValue::Superscript => "Superscript",
                        NodeValue::Link(_) => "Link",
                        NodeValue::Image(_) => "Image",
                        NodeValue::FootnoteReference(_) => "FootnoteReference",
                    };
                    format!("start_line: {}: {}", ast.start_line, name)
                })
                .collect::<Vec<String>>();
            f.debug_list().entries(items).finish()
        }
    }
    AstDebug(asts.to_vec())
}

impl Instruction {
    pub fn set_line_start(&mut self, line_start: usize) {
        self.name.set_line_start(line_start)
    }
}
