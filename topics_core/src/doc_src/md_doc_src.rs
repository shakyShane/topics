use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_err::DocError;
use crate::doc_src::DocSrcImpl;
use crate::items::{Item, ItemWrap};
use multi_doc::MultiDoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct MdDocSource {
    pub input_file: Option<PathBuf>,
    pub file_content: String,
    pub doc_src_items: MultiDoc,
}

impl DocSrcImpl for MdDocSource {
    fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocError::PathRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let items = MultiDoc::from_str(&file_str)?;
        let new_self = Self {
            input_file: Some(pb.clone()),
            file_content: file_str,
            doc_src_items: items,
        };
        Ok(new_self)
    }
}

impl FromStr for MdDocSource {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = MultiDoc::from_md_str(&s)?;
        Ok(Self {
            input_file: None,
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
}

#[derive(Clone, Debug)]
enum Element {
    Heading {
        level: usize,
        content: String,
    },
    List {
        items: Vec<ListItem>,
    },
    Paragraph {
        content: String,
    },
    Text {
        content: String,
    },
    CodeBlock {
        params: Option<String>,
        content: String,
    },
}

#[derive(Clone, Debug)]
struct ListItem(Vec<Element>);

///
/// Lex 1 source file
///
/// The purpose of this pass is to collect only relevant
/// information for the following 'parse' phase.
///
/// That means we can skip things like deeply nested lists
/// etc and opt for a simpler non-recursive algorithm
///
fn lex_one(item_src: &str) -> DocResult<Vec<Element>> {
    let mut items: Vec<Element> = vec![];
    let mut temp_items: Vec<Element> = vec![];
    let mut temp_list_items: Vec<ListItem> = vec![];
    let mut buffer = String::new();

    for item in Parser::new_ext(item_src, Options::empty()) {
        match item {
            Event::Start(Tag::List(_)) => {
                items.extend(temp_items.clone());
                temp_items.clear();
                temp_list_items.clear();
            }
            Event::End(Tag::List(_l)) => {
                items.push(Element::List {
                    items: temp_list_items.clone(),
                });
                temp_list_items.clear();
            }
            Event::End(Tag::Item) => {
                if !buffer.is_empty() {
                    temp_items.push(Element::Text {
                        content: buffer.to_string(),
                    });
                }
                temp_list_items.push(ListItem(temp_items.clone()));
                temp_items.clear();
                buffer.clear();
            }
            Event::End(Tag::Heading(h)) => {
                temp_items.push(Element::Heading {
                    level: h as usize,
                    content: buffer.to_string(),
                });
                buffer.clear();
            }
            Event::Start(Tag::Paragraph) => {
                buffer.clear();
            }
            Event::End(Tag::Paragraph) => {
                temp_items.push(Element::Paragraph {
                    content: buffer.to_string(),
                });
                buffer.clear();
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                if !buffer.is_empty() {
                    println!("BUFFER WAS NOT EMPTY before code block started={}", buffer);
                    temp_items.push(Element::Text {
                        content: buffer.to_string(),
                    });
                    buffer.clear()
                }
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(args))) => {
                temp_items.push(Element::CodeBlock {
                    content: buffer.to_string(),
                    params: Some(args.to_string()),
                });
                buffer.clear();
            }
            Event::Text(t) => {
                buffer.push_str(&t);
            }
            Event::Code(_) => {}
            Event::Html(_) => {}
            Event::FootnoteReference(_) => {}
            Event::SoftBreak => {
                buffer.push('\n');
            }
            Event::HardBreak => {}
            Event::Rule => {}
            Event::TaskListMarker(_) => {}
            _t => {}
        }
    }
    items.extend(temp_items); // may be empty at this point
    Ok(items)
}

pub fn parse_md_str(input: &str) -> DocResult<Vec<Item>> {
    let src = MdDocSource::from_str(input)?;
    let mut items: Vec<Item> = vec![];
    for item in &src.doc_src_items.items {
        let elements = lex_one(&item.content)?;
        let inner_items = parse_elements(&elements)?;
        items.extend(inner_items);
    }
    Ok(items)
}

fn parse_elements(elements: &[Element]) -> DocResult<Vec<Item>> {
    let mut items: Vec<Item> = vec![];
    let mut kind: Option<Item> = None;
    let mut prev_heading2: Option<Element> = None;
    for elem in elements {
        match elem {
            Element::Heading { level: 1, content } => {
                let split = content.splitn(2, ':').collect::<Vec<&str>>();
                match (split.get(0), split.get(1)) {
                    (Some(kind_str), Some(rest)) => {
                        if let Ok(mut item) = Item::from_str(kind_str) {
                            item.set_name(rest.trim());
                            kind = Some(item);
                        }
                    }
                    _ => todo!("invalid title {:?}", split),
                }
            }
            h @ Element::Heading { .. } => {
                prev_heading2 = Some(h.clone());
            }
            Element::CodeBlock {
                params: Some(p),
                content,
            } => {
                if let Some(Item::Command(cmd)) = kind.as_mut() {
                    match code_fence::parse_code_fence_args(&p) {
                        Ok(Some(code_fence::Cmd::Command(inner))) => {
                            // we only assign this code block if it has ```shell command ...
                            cmd.command = content.to_string();
                            cmd.cwd = inner.cwd;
                        }
                        _a => {
                            todo!("handle parsing code-block inline args")
                        }
                    }
                }
            }
            Element::List { items } => {
                if let Some(Element::Heading { content, level: 2 }) = &prev_heading2 {
                    if let Some(Item::Topic(top)) = kind.as_mut() {
                        match content.as_str() {
                            "Dependencies" | "Dependencies:" => {
                                for item in items {
                                    let parsed = parse_item_from_list_items(&item.0);
                                    if let Some(item_wrap) = parsed {
                                        top.deps.push(item_wrap);
                                    }
                                }
                            }
                            "Steps" | "Steps:" => {
                                for item in items {
                                    let parsed = parse_item_from_list_items(&item.0);
                                    if let Some(item_wrap) = parsed {
                                        top.steps.push(item_wrap);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                // println!("prev heading {:?}", prev_heading2);
            }
            _ => {}
        }
    }
    if let Some(item) = kind {
        items.push(item)
    }
    Ok(items)
}

///
/// There are special semantics for parsing an inline list item
///
/// Things we care about
///
fn parse_item_from_list_items(items: &[Element]) -> Option<ItemWrap> {
    // 1. if there's a single text node, this is a named reference
    if let (1, Some(Element::Text { content })) = (items.len(), items.get(0)) {
        return parse_inline_kind(content)
            .map(ItemWrap::Item)
            .or_else(|| Some(ItemWrap::Named(content.to_string())));
    }
    if let (1, Some(Element::Paragraph { content })) = (items.len(), items.get(0)) {
        return parse_inline_kind(content)
            .map(ItemWrap::Item)
            .or_else(|| Some(ItemWrap::Named(content.to_string())));
    }

    // 2. Instruction: name pattern. This should be 1 kind_line + n trailing lines.
    if let (_len, Some(Element::Paragraph { content })) = (items.len(), items.get(0)) {
        if let Some(Item::Instruction(mut ins)) = parse_inline_kind(content) {
            let mut instruction_lines = vec![];
            items.iter().skip(1).for_each(|rem| match rem {
                Element::Paragraph { content } => instruction_lines.push(content.to_string()),
                Element::Text { content } => instruction_lines.push(content.to_string()),
                _ => {}
            });
            ins.instruction = instruction_lines.join("\n");
            return Some(ItemWrap::Item(Item::Instruction(ins)));
        }
    }

    None
}

fn parse_inline_kind(input: &str) -> Option<Item> {
    // split by lines first - inline kind names must be a single line
    let lines = input.splitn(2, '\n').collect::<Vec<&str>>();

    // kind + name + other
    if let (Some(first), maybe_rest) = (lines.get(0), lines.get(1)) {
        if let Some(Item::Instruction(inst)) = split_first_line(first).as_mut() {
            if let Some(rest) = maybe_rest {
                inst.instruction = rest.to_string();
            }
            return Some(Item::Instruction(inst.clone()));
        }
    }

    // kind + name only
    if let (Some(_first), None) = (lines.get(0), lines.get(1)) {}

    None
}

fn split_first_line(first_line_input: &str) -> Option<Item> {
    let split = first_line_input.splitn(2, ':').collect::<Vec<&str>>();
    match (split.get(0), split.get(1)) {
        (Some(kind_str), Some(rest)) => {
            if let Ok(mut item) = Item::from_str(kind_str) {
                item.set_name(rest.trim());
                Some(item)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[test]
fn test_single_named_reference() {
    let list = vec![Element::Text {
        content: "Dep 1".to_string(),
    }];
    let output = parse_item_from_list_items(&list);
    assert_eq!(output, Some(ItemWrap::Named(String::from("Dep 1"))));
}

#[test]
fn test_inline_instruction() {
    let input = r#"
# Topic: do work

## Dependencies 

- Instruction: Login

  Now you can login into your account without worrying
- Dep 2
- Dep 2

## Steps
- step 1
- step 2
- step 3
- Instruction: Wave goodbye

  This is just an indented guide
    "#;

    let elements = lex_one(input).unwrap();
    let items = parse_elements(&elements).unwrap();
    assert_eq!(items.get(0).unwrap().name(), "do work");
}

pub mod code_fence {
    use crate::items::CommandInlineArgs;
    use structopt::StructOpt;

    #[derive(Debug, structopt::StructOpt)]
    pub enum Cmd {
        Command(CommandInlineArgs),
        Verify,
    }
    #[derive(Debug, structopt::StructOpt)]
    pub struct CodeBlock {
        #[structopt(subcommand)]
        pub cmd: Cmd,
    }

    pub(crate) fn parse_code_fence_args(input: &str) -> anyhow::Result<Option<Cmd>> {
        if let Ok(words) = split_args(&input) {
            if words.len() > 1 {
                if let Ok(cb) = Cmd::from_iter_safe(&words) {
                    return Ok(Some(cb));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn split_args(input: &str) -> anyhow::Result<Vec<String>> {
        shellwords::split(input).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let input = r#"
# Command: Run unit tests <br>command</br>
## This is a description

```shell command
echo "About to install ${MIN_VERSION}"
yarn build:static && \
yarn export
```
---
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = parse_md_str(&src.file_content)?;
        assert_eq!(items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_cmd_item() -> anyhow::Result<()> {
        let input = r#"
# Command: Run unit tests <br>command</br>

## This is a description

```shell command --cwd=/containers/www/client
echo "About to install ${MIN_VERSION}"
yarn build:static && yarn export
```

```shell
echo just another code block that should not be counted
```
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = parse_md_str(&src.file_content)?;
        assert_eq!(items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_topic_item_with_inline_command() -> anyhow::Result<()> {
        let input = r#"
# Topic: Run unit tests

## Dependencies

- Instruction: Get access to GH

  with a newline
- Dep 2

## Steps

- Command: Unit tests jest command

  ```shell command --cwd="containers/www/client"
  jest --runInBand
  rm -rf test/out
  ```
- Check output
  
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = parse_md_str(&src.file_content)?;
        match items.get(0).unwrap() {
            Item::Topic(topic) => {
                assert_eq!(topic.deps.len(), 2);
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}
