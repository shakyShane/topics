use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_err::DocError;
use crate::doc_src::{DocSrcImpl, MdSrc};
use crate::items::{Item, ItemWrap};
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, ListType};
use multi_doc::MultiDoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

use std::convert::TryFrom;
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

impl<'a> From<&'a MdDocSource> for &'a str {
    fn from(mds: &'a MdDocSource) -> Self {
        &mds.file_content
    }
}

pub fn parse_md_str(input: &str) -> DocResult<Vec<Item>> {
    let md_src_doc = MdDocSource::from_str(input)?;
    let md_src = MdSrc::new(&md_src_doc);
    md_src.parse();
    let items = md_src.items();
    Ok(items)
}

// pub fn parse_elements(elements: &[Element]) -> DocResult<Vec<Item>> {
//     let mut items: Vec<Item> = vec![];
//     let mut kind: Option<Item> = None;
//     let mut prev_heading2: Option<Element> = None;
//     for elem in elements {
//         match elem {
//             Element::Heading { level: 1, content } => {
//                 let split = content.splitn(2, ':').collect::<Vec<&str>>();
//                 match (split.get(0), split.get(1)) {
//                     (Some(kind_str), Some(rest)) => {
//                         if let Ok(mut item) = Item::from_str(kind_str) {
//                             item.set_name(rest.trim());
//                             kind = Some(item);
//                         }
//                     }
//                     _ => todo!("invalid title {:?}", split),
//                 }
//             }
//             h @ Element::Heading { .. } => {
//                 prev_heading2 = Some(h.clone());
//             }
//             Element::CodeBlock {
//                 params: Some(p),
//                 content,
//             } => {
//                 if let Some(Item::Command(cmd)) = kind.as_mut() {
//                     cmd.with_content(content);
//                     cmd.with_cli_params(&p);
//                 }
//                 if let Some(Item::DependencyCheck(dep_check)) = kind.as_mut() {
//                     use code_fence::Cmd::*;
//                     match code_fence::parse_code_fence_args(&p) {
//                         Ok(Some(Verify(_))) => {
//                             // we only assign this code block if it has ```shell verify ...
//                             dep_check.verify = content.to_string();
//                         }
//                         Ok(Some(AutoFix(_))) => {
//                             // we only assign this code block if it has ```shell autofix ...
//                             dep_check.autofix = Some(content.to_string());
//                         }
//                         _a => {
//                             todo!("handle parsing code-block inline args")
//                         }
//                     }
//                 }
//             }
//             Element::List { items } => {
//                 if let Some(Element::Heading { content, level: 2 }) = &prev_heading2 {
//                     if let Some(Item::Topic(topic)) = kind.as_mut() {
//                         match content.as_str() {
//                             "Dependencies" | "Dependencies:" => {
//                                 for item in items {
//                                     let parsed = parse_item_from_list_items(&item.0);
//                                     if let Some(item_wrap) = parsed {
//                                         topic.deps.push(item_wrap);
//                                     }
//                                 }
//                             }
//                             "Steps" | "Steps:" => {
//                                 for item in items {
//                                     let parsed = parse_item_from_list_items(&item.0);
//                                     if let Some(item_wrap) = parsed {
//                                         topic.steps.push(item_wrap);
//                                     }
//                                 }
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 // println!("prev heading {:?}", prev_heading2);
//             }
//             _ => {}
//         }
//     }
//     if let Some(item) = kind {
//         items.push(item)
//     }
//     Ok(items)
// }

///
/// There are special semantics for parsing an inline list item
///
/// Things we care about
///
// fn parse_item_from_list_items(items: &[Element]) -> Option<ItemWrap> {
//     // 1. if there's a single text node, this is a named reference
//     if let (1, Some(Element::Text { content })) = (items.len(), items.get(0)) {
//         return parse_inline_kind(content)
//             .map(ItemWrap::Item)
//             .or_else(|| Some(ItemWrap::Named(content.to_string())));
//     }
// if let (1, Some(Element::Paragraph { content })) = (items.len(), items.get(0)) {
//     return parse_inline_kind(content)
//         .map(ItemWrap::Item)
//         .or_else(|| Some(ItemWrap::Named(content.to_string())));
// }

// 2. <kind>: <name> pattern. This should be 1 line of kind + name and then `n` trailing lines.
// if let (_len, Some(Element::Paragraph { content })) = (items.len(), items.get(0)) {
//     match parse_inline_kind(content) {
//         Some(Item::Instruction(mut ins)) => {
//             let mut instruction_lines = vec![];
//             items.iter().skip(1).for_each(|rem| match rem {
//                 Element::Paragraph { content } => instruction_lines.push(content.to_string()),
//                 Element::Text { content } => instruction_lines.push(content.to_string()),
//                 _ => {}
//             });
//             ins.instruction = instruction_lines.join("\n");
//             return Some(ItemWrap::Item(Item::Instruction(ins)));
//         }
//         Some(Item::Command(mut cmd)) => {
//             items.iter().skip(1).for_each(|rem| match rem {
//                 Element::CodeBlock {
//                     content,
//                     params: Some(p),
//                 } => {
//                     cmd.with_content(content);
//                     cmd.with_cli_params(p);
//                 }
//                 Element::CodeBlock { content: _, params } => {
//                     if params.is_none() {
//                         eprintln!(
//                             "TODO - missing inline params on codefence from inline COMMAND item. "
//                         )
//                     }
//                 }
//                 _ => {}
//             });
//             return Some(ItemWrap::Item(Item::Command(cmd)));
//         }
//         Some(Item::DependencyCheck(mut dep_check)) => {
//             items.iter().skip(1).for_each(|rem| match rem {
//                 Element::CodeBlock {
//                     content,
//                     params: Some(p),
//                 } => {
//                     dep_check.with_content(content, p);
//                 }
//                 Element::CodeBlock { content: _, params } => {
//                     if params.is_none() {
//                         eprintln!(
//                             "TODO - missing inline params on codefence from inline DependencyCheck item. "
//                         )
//                     }
//                 }
//                 _ => {}
//             });
//             return Some(ItemWrap::Item(Item::DependencyCheck(dep_check)));
//         }
//         _ => { /* noop */ }
//     }
// }

// None
// }

pub fn parse_inline_kind(input: &str) -> Option<Item> {
    // split by lines first - inline kind names must be a single line
    let lines = input.splitn(2, '\n').collect::<Vec<&str>>();

    // kind + name + other
    if let (Some(first), maybe_rest) = (lines.get(0), lines.get(1)) {
        match split_first_line(first).as_mut() {
            Some(Item::Instruction(inst)) => {
                return Some(Item::Instruction(inst.clone()));
            }
            Some(Item::Command(cmd)) => {
                return Some(Item::Command(cmd.clone()));
            }
            Some(Item::DependencyCheck(dep_check)) => {
                return Some(Item::DependencyCheck(dep_check.clone()));
            }
            Some(Item::Topic(topic)) => {
                return Some(Item::Topic(topic.clone()));
            }
            Some(_v) => {
                todo!("todo inline list item");
            }
            None => {}
        }
    }

    // kind + name only
    if let (Some(_first), None) = (lines.get(0), lines.get(1)) {
        // todo!("kind + name only in a list, needs to be solved")
    }

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

pub mod code_fence {
    use crate::items::{AutoFixInlineArgs, CommandInlineArgs, VerifyInlineArgs};
    use structopt::StructOpt;

    #[derive(Debug, structopt::StructOpt)]
    pub enum Cmd {
        Command(CommandInlineArgs),
        Verify(VerifyInlineArgs),
        #[structopt(alias = "autofix")]
        AutoFix(AutoFixInlineArgs),
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
    use crate::items::{Command, DependencyCheck};

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
    fn test_dep_check_item() -> anyhow::Result<()> {
        let input = r#"
# Dependency Check: Node JS installed globally

Node JS is required and should be on version 12.0

```command verify
node -v
```

This can be auto fixed with the following command

```command autofix
nvm use 12
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
- Dependency Check: Global Node

    ```shell verify
    node -v
    ```
    
    ```shell autofix
    nvm i 12
    ```
  
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = parse_md_str(&src.file_content)?;
        match items.get(0).unwrap() {
            Item::Topic(topic) => {
                assert_eq!(topic.deps.len(), 2);
            }
            _ => unreachable!(),
        };
        match items.get(0).unwrap() {
            Item::Topic(topic) => {
                assert_eq!(topic.steps.len(), 3);
                let first_item = topic.steps.get(0).unwrap();
                match first_item {
                    ItemWrap::Item(Item::Command(Command { cwd, name, .. })) => {
                        assert_eq!(cwd.0, PathBuf::from("containers/www/client"));
                        assert_eq!(name, &String::from("Unit tests jest command"));
                    }
                    _ => unreachable!(),
                }
                let last_item = topic.steps.get(2).unwrap();
                match last_item {
                    ItemWrap::Item(Item::DependencyCheck(DependencyCheck {
                        verify, name, ..
                    })) => {
                        assert_eq!(name, &String::from("Global Node"));
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}
