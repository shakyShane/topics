use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_err::DocError;
use crate::doc_src::DocSrcImpl;
use crate::items::Item;
use multi_doc::{MultiDoc, SingleDoc};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::collections::HashMap;
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
        let items = MultiDoc::from_str(&s)?;
        Ok(Self {
            input_file: None,
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
}

impl MdDocSource {
    pub fn to_items(&self) -> DocResult<Vec<Item>> {
        self.doc_src_items
            .items
            .iter()
            .map(|item_src| parse_one(&self, item_src))
            .collect()
    }
}

#[derive(Debug)]
enum Collecting {
    Heading,
    CodeBlock,
    None,
}

#[derive(Debug)]
enum Element {
    Heading {
        level: usize,
        content: String,
    },
    CodeBlock {
        params: HashMap<String, Option<String>>,
        content: String,
    },
}

fn parse_one(_doc: &MdDocSource, item_src: &SingleDoc) -> DocResult<Item> {
    let mut items: Vec<Element> = vec![];
    let mut collecting = Collecting::None;
    let mut buffer = String::new();

    for evt in Parser::new_ext(&item_src.content, Options::empty()) {
        match evt {
            Event::Text(t) => match collecting {
                Collecting::Heading | Collecting::CodeBlock => buffer.push_str(&t),
                Collecting::None => {}
            },
            Event::End(a) => {
                match a {
                    Tag::Heading(level) => match collecting {
                        Collecting::Heading => {
                            items.push(Element::Heading {
                                level: level as usize,
                                content: buffer.to_string(),
                            });
                            buffer.clear();
                            collecting = Collecting::None;
                        }
                        _ => unreachable!(),
                    },
                    Tag::CodeBlock(code) => match code {
                        CodeBlockKind::Indented => {}
                        CodeBlockKind::Fenced(fence_args) => match collecting {
                            Collecting::CodeBlock => {
                                items.push(Element::CodeBlock {
                                    params: Default::default(),
                                    content: buffer.to_string(),
                                });
                                buffer.clear();
                                collecting = Collecting::None;
                            }
                            _ => unreachable!(),
                        },
                    },
                    _ => {
                        // noop
                    }
                }
            }
            Event::Start(a) => match a {
                Tag::Paragraph => {}
                Tag::Heading(_num) => collecting = Collecting::Heading,
                Tag::BlockQuote => {}
                Tag::CodeBlock(code) => match code {
                    CodeBlockKind::Indented => {
                        // println!("code indented");
                    }
                    CodeBlockKind::Fenced(_) => {
                        // if fence_args.contains("@command") {
                        //     fence_args
                        //         .split_whitespace()
                        //         .filter(|c| !c.starts_with("@command"))
                        //         .for_each(|chunk| {
                        //             let mut left = None;
                        //             let mut right = None;
                        //             for c in chunk.split("=") {
                        //                 if left.is_none() {
                        //                     left = Some(c)
                        //                 } else {
                        //                     right = Some(c)
                        //                 }
                        //             }
                        //             match (left, right) {
                        //                 (Some(left), None) => {
                        //                     println!("left ONLY={}", left);
                        //                 }
                        //                 (Some("@cwd"), Some(v)) => {
                        //                     println!("cwd={}", v);
                        //                     cwd = Some(v.to_string());
                        //                 }
                        //                 (Some("@cwd"), None) => {
                        //                     println!("MISSING CWD VALUE");
                        //                 }
                        //                 _ => println!("other"),
                        //             }
                        //         });
                        // }
                        collecting = Collecting::CodeBlock
                    }
                },
                Tag::List(_) => {}
                Tag::Item => {}
                Tag::FootnoteDefinition(_) => {}
                Tag::Table(_) => {}
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Emphasis => {}
                Tag::Strong => {}
                Tag::Strikethrough => {}
                Tag::Link(_, _, _) => {}
                Tag::Image(_, _, _) => {}
            },
            _ => {
                // println!("other")
            }
        }
    }
    dbg!(items);
    Ok(Item::Topic(Default::default()))
}

fn arg_hash_map(args: &str) -> HashMap<String, Option<String>> {
    args.split_whitespace()
        .filter_map(|chunk| {
            let mut left = None;
            let mut right = None;
            for (index, c) in chunk.split("=").enumerate() {
                if index == 0 {
                    if !c.is_empty() {
                        left = Some(c);
                    }
                } else {
                    right = Some(c);
                }
            }
            match (left, right) {
                (Some(left), Some(right)) => Some((left.to_string(), Some(right.to_string()))),
                (Some(left), None) => Some((left.to_string(), None)),
                (_, _) => {
                    println!("invalid");
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let input = r#"
# Command: Run unit tests <br>command</br>
## This is a description

```shell --kind=command
echo "About to install ${MIN_VERSION}"
yarn build:static && \
yarn export
```
---
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = src.to_items()?;
        // dbg!(items);
        Ok(())
    }

    #[test]
    fn test_args() {
        // enum Cmd {
        //     Command
        // }
        #[derive(Structopt)]
        struct CodeBlock {
            #[structopt(subcommand)]
            cmd()
        }
        let input = "shell command ";
        let as_hash = arg_hash_map(input);
        dbg!(as_hash);
    }
}
