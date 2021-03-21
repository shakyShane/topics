use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_err::DocError;
use crate::doc_src::DocSrcImpl;
use crate::items::{CommandInlineArgs, Item};
use multi_doc::{MultiDoc, SingleDoc};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

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

// impl MdDocSource {
//     pub fn to_items(&self) -> DocResult<Vec<Item>> {
//         self.doc_src_items
//             .items
//             .iter()
//             .map(|item_src| parse_one(&self, item_src))
//             .collect()
//     }
// }

#[derive(Debug)]
enum Collecting {
    Paragraph,
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
    Paragraph {
        content: String,
    },
    CodeBlock {
        params: Option<String>,
        content: String,
    },
}

fn lex_one(_doc: &MdDocSource, item_src: &SingleDoc) -> DocResult<Vec<Element>> {
    let mut items: Vec<Element> = vec![];
    let mut collecting = Collecting::None;
    let mut buffer = String::new();
    // let mut options = Options::empty();
    // options.insert(Options::ENABLE_SMART_PUNCTUATION);

    for evt in Parser::new_ext(&item_src.content, Options::empty()) {
        match evt {
            Event::Text(t) => match collecting {
                Collecting::Heading | Collecting::CodeBlock | Collecting::Paragraph => {
                    buffer.push_str(&t)
                }
                Collecting::None => {}
            },
            Event::End(a) => {
                match a {
                    Tag::Paragraph => match collecting {
                        Collecting::Paragraph => {
                            items.push(Element::Paragraph {
                                content: buffer.to_string(),
                            });
                            buffer.clear();
                        }
                        _ => {}
                    },
                    Tag::Item => {
                        // buffer.clear();
                    }
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
                                    params: if !fence_args.is_empty() {
                                        Some(fence_args.to_string())
                                    } else {
                                        None
                                    },
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
                Tag::Paragraph => collecting = Collecting::Paragraph,
                Tag::Heading(_num) => collecting = Collecting::Heading,
                Tag::BlockQuote => {}
                Tag::CodeBlock(code) => match code {
                    CodeBlockKind::Indented => {}
                    CodeBlockKind::Fenced(_) => collecting = Collecting::CodeBlock,
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
    Ok(items)
}

pub fn parse_md(input: &str) -> DocResult<Vec<Item>> {
    #[derive(Debug, structopt::StructOpt)]
    enum Cmd {
        Command(CommandInlineArgs),
        Verify,
    }
    #[derive(Debug, structopt::StructOpt)]
    struct CodeBlock {
        #[structopt(subcommand)]
        cmd: Cmd,
    }
    let src = MdDocSource::from_str(input)?;
    let mut items: Vec<Item> = vec![];
    for item in &src.doc_src_items.items {
        let elements = lex_one(&src, &item)?;
        let mut kind: Option<Item> = None;
        for elem in elements {
            match elem {
                Element::Heading { level: 1, content } => {
                    let split = content.splitn(2, ":").collect::<Vec<&str>>();
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
                Element::CodeBlock {
                    params: Some(p), ..
                } => {
                    let words = shellwords::split(&p);
                    match kind.as_mut() {
                        Some(Item::Command(cmd)) => {
                            if let Ok(words) = words {
                                if words.len() > 1 {
                                    let cb = CodeBlock::from_iter_safe(words);
                                    if let Ok(cb) = cb {
                                        match cb.cmd {
                                            Cmd::Command(inline) => {
                                                cmd.cwd = inline.cwd;
                                            }
                                            Cmd::Verify => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if let Some(item) = kind {
            items.push(item)
        }
    }
    Ok(items)
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
        let items = parse_md(&src.file_content)?;
        assert_eq!(items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_args() -> anyhow::Result<()> {
        let input = r#"
# Command: Run unit tests <br>command</br>

## This is a description

```shell command --cwd=/containers/www/client
echo "About to install ${MIN_VERSION}"
yarn build:static && yarn export
```

```shell
echo just another code block
```
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = parse_md(&src.file_content)?;
        dbg!(&items);
        assert_eq!(items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_list() -> anyhow::Result<()> {
        let src = MdDocSource::from_str(include_str!("../../../fixtures/md/topics.md"))?;
        let items = lex_one(&src, &src.doc_src_items.items.get(0).unwrap())?;
        dbg!(&items);
        // assert_eq!(items.len(), 1);
        Ok(())
    }
}
