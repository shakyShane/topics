use crate::doc::DocResult;
use crate::doc_src::{Element, MdElements};
use comrak::nodes::{AstNode, NodeCodeBlock, NodeHeading, NodeValue};
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

impl Deref for MdElements {
    type Target = Vec<Element>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

fn str_to_elements(s: &str) -> Vec<Element> {
    let arena = Arena::new();
    let root = parse_document(&arena, s, &ComrakOptions::default());
    let mut elements: Vec<Element> = vec![];

    fn iter_nodes<'a>(node: &'a AstNode<'a>, elements: &mut Vec<Element>) {
        let ast = node.data.borrow();
        match &ast.value {
            NodeValue::Heading(NodeHeading { level: 1, .. }) => {
                let t = collect_single_line_text(node);
                elements.push(Element::h1(t));
            }
            NodeValue::Heading(NodeHeading { level: 2, .. }) => {}
            NodeValue::Heading(NodeHeading { level: 3, .. }) => {}
            NodeValue::Paragraph => {
                println!("++p");
                let p = collect_paragraph(node);
                println!("\t\t|{}|", p);
                println!("--p");
            }
            NodeValue::HtmlBlock(html_block) => {
                let html = std::str::from_utf8(&html_block.literal).unwrap();
                println!("++html",);
                println!("\t\t|{}|", html);
                println!("--html",);
            }
            NodeValue::CodeBlock(cb @ NodeCodeBlock { fenced: true, .. }) => {
                let NodeCodeBlock { literal, info, .. } = cb;

                // `trim_end` is here because the comrak parser adds a new-line to the
                // end of code blocks, which may be spec-compliant, but it's possibly
                // something we'd rather not forget about later
                let literal = std::str::from_utf8(&literal).unwrap().trim_end();
                let info = std::str::from_utf8(&info).unwrap();

                if info.is_empty() {
                    elements.push(Element::code_block_without_params(literal));
                } else {
                    elements.push(Element::code_block(literal, Some(info)));
                }
            }
            _ => {
                for c in node.children() {
                    iter_nodes(c, elements);
                }
            }
        }
    }

    iter_nodes(root, &mut elements);

    elements
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

///
/// Collect paragraph as-is
///
fn collect_paragraph<'a>(node: &'a AstNode<'a>) -> String {
    use std::io::{self, Write};
    let mut output: Vec<u8> = Vec::new();
    let html = format_html(&node, &ComrakOptions::default(), &mut output);
    std::str::from_utf8(&output).unwrap().to_string()
}
