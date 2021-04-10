use crate::doc::DocResult;
use crate::doc_src::{parse_inline_kind, MdElements};
use crate::items::{Instruction, Item};
use comrak::nodes::{Ast, AstNode, NodeCodeBlock, NodeHeading, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use std::ops::Deref;
use std::str::FromStr;

impl FromStr for MdElements {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = str_to_elements(s);
        Ok(Self { elements })
    }
}

fn process_node<'a>(node: &'a AstNode<'a>) -> Vec<Item> {
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

    if let Some(Item::Instruction(Instruction { ast, .. })) = kind.as_mut() {
        let cloned = node.to_owned();
        let mut items: Vec<Ast> = vec![];
        for n in cloned.children() {
            let ast = n.data.clone().into_inner();
            items.push(ast);
        }
        *ast = items;
    }

    if let Some(kind) = kind {
        items.push(kind)
    }

    items
}

fn str_to_elements(s: &str) -> Vec<Item> {
    let arena = Arena::new();
    let root = parse_document(&arena, s, &ComrakOptions::default());
    let items = process_node(root);

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

///
/// Single-line text items are identifiers and follow special rules.
///
fn collect_single_line_text<'a>(node: &'a AstNode<'a>) -> String {
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
