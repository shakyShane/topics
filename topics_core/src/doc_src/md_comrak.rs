use crate::doc::DocResult;
use crate::doc_src::{parse_inline_kind, MdElements};
use crate::items::{Command, Instruction, Item};
use comrak::nodes::{Ast, AstNode, NodeCodeBlock, NodeHeading, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

fn process_node<'a>(node: &'a AstNode<'a>, path: &mut Vec<usize>) -> Vec<Item> {
    let mut kind: Option<Item> = None;
    let mut items: Vec<Item> = vec![];
    let first = node.children().take(1).nth(0);
    let other = node.children().skip(1);

    // process the very first item, expected to be a heading, but maybe not?
    if let Some(node) = first {
        let ast = node.data.borrow();
        match &ast.value {
            NodeValue::Heading(NodeHeading { level: 1, .. }) => {
                let start_line = ast.start_line;
                let t = collect_single_line_text(node);
                let item = parse_inline_kind(&t);
                kind = item;
                if let Some(item) = kind.as_mut() {
                    item.set_line_start(start_line as usize)
                }
            }
            _ => {
                todo!("handle other 'first' elements?")
            }
        }
    }

    if let Some(Item::Instruction(inst)) = kind.as_mut() {
        let Instruction {
            name,
            ast_len,
            ast_start,
        } = inst;
        println!("{}", **name);
        *ast_start = path.to_vec();
        *ast_len = node.children().count();
    }

    // if let Some(Item::Command(c @ Command { .. })) = kind.as_mut() {
    //     dbg!(c);
    // }

    if let Some(kind) = kind {
        items.push(kind)
    }

    items
}

pub(crate) fn to_elements(md: &'_ MdElements<'_>) -> Vec<Item> {
    let mut path = vec![0];
    let items = process_node(&md.root, &mut path);

    // for node in root.children() {
    //     let ast = node.data.borrow();
    //
    //     if let Some(Item::Instruction(Instruction { ast, .. })) = kind.as_mut() {
    //         println!("skipping before it's an instruction");
    //         continue;
    //     }
    //
    //     match &ast.value {
    //         NodeValue::Heading(NodeHeading { level: 1, .. }) => {
    //             let t = collect_single_line_text(node);
    //             let item = parse_inline_kind(&t);
    //             kind = item;
    //             // println!("got header.. = {} ", t);
    //             // println!("got item.. = {:?} ", item);
    //         }
    //         NodeValue::Heading(NodeHeading { level: 2, .. }) => {}
    //         NodeValue::Heading(NodeHeading { level: 3, .. }) => {}
    //         NodeValue::Paragraph => {
    //             // println!("++p");
    //             // let ast = node.data.clone().into_inner();
    //             // println!("--p");
    //         }
    //         NodeValue::HtmlBlock(html_block) => {
    //             let html = std::str::from_utf8(&html_block.literal).unwrap();
    //             // println!("++html",);
    //             // println!("\t\t|{}|", html);
    //             // println!("--html",);
    //         }
    //         NodeValue::CodeBlock(cb @ NodeCodeBlock { fenced: true, .. }) => {
    //             let NodeCodeBlock { literal, info, .. } = cb;
    //
    //             // `trim_end` is here because the comrak parser adds a new-line to the
    //             // end of code blocks, which may be spec-compliant, but it's possibly
    //             // something we'd rather not forget about later
    //             let literal = std::str::from_utf8(&literal).unwrap().trim_end();
    //             let info = std::str::from_utf8(&info).unwrap();
    //
    //             // if info.is_empty() {
    //             //     items.push(Element::code_block_without_params(literal));
    //             // } else {
    //             //     items.push(Element::code_block(literal, Some(info)));
    //             // }
    //         }
    //         _ => {}
    //     }
    // }
    //
    // if let Some(item) = kind {
    //     items.push(item);
    // }

    items
}

pub fn debug_ast(asts: &[Ast]) -> impl Debug {
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

///
/// Single-line text items are identifiers and follow special rules.
///
pub(crate) fn collect_single_line_text<'a>(node: &'a AstNode<'a>) -> String {
    node.children()
        .filter_map(|n| match &n.data.borrow().value {
            NodeValue::Text(t) => Some(std::str::from_utf8(t).unwrap().to_string()),
            // todo, preserve this information?
            NodeValue::Code(t) => Some(std::str::from_utf8(t).unwrap().to_string()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("")
}
