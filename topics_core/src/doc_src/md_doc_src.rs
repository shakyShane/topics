use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_err::DocError;
use crate::doc_src::DocSrcImpl;
use crate::items::Item;
use multi_doc::{MultiDoc, SingleDoc};
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

fn parse_one(_doc: &MdDocSource, item_src: &SingleDoc) -> DocResult<Item> {
    let _kind = "";
    let mut name = "".to_string();
    let mut collecting_name = false;

    let mut command = "".to_string();
    let mut capturing_command = false;
    let mut cwd: Option<String> = None;

    for evt in Parser::new_ext(&item_src.content, Options::empty()) {
        match evt {
            Event::Text(t) => {
                if collecting_name {
                    name.push_str(&t)
                }
                if capturing_command {
                    command.push_str(&t)
                }
                // println!("t=>{}", t);
            }
            Event::End(a) => {
                match a {
                    Tag::Heading(1) => collecting_name = false,
                    Tag::CodeBlock(code) => match code {
                        CodeBlockKind::Indented => {}
                        CodeBlockKind::Fenced(_fence_args) => capturing_command = false,
                    },
                    _ => {
                        // noop
                    }
                }
            }
            Event::Start(a) => match a {
                Tag::Paragraph => {}
                Tag::Heading(1) => {
                    if !collecting_name && name.is_empty() {
                        collecting_name = true
                    }
                }
                Tag::Heading(_other) => {}
                Tag::BlockQuote => {}
                Tag::CodeBlock(code) => match code {
                    CodeBlockKind::Indented => {
                        println!("code indented");
                    }
                    CodeBlockKind::Fenced(fence_args) => {
                        if !capturing_command && command.is_empty() {
                            if fence_args.contains("@command") {
                                fence_args
                                    .split_whitespace()
                                    .filter(|c| !c.starts_with("@command"))
                                    .for_each(|chunk| {
                                        let mut left = None;
                                        let mut right = None;
                                        for c in chunk.split("=") {
                                            if left.is_none() {
                                                left = Some(c)
                                            } else {
                                                right = Some(c)
                                            }
                                        }
                                        match (left, right) {
                                            (Some(left), None) => {
                                                println!("left ONLY={}", left);
                                            }
                                            (Some("@cwd"), Some(v)) => {
                                                println!("cwd={}", v);
                                                cwd = Some(v.to_string());
                                            }
                                            (Some("@cwd"), None) => {
                                                println!("MISSING CWD VALUE");
                                            }
                                            _ => println!("other"),
                                        }
                                    });
                                capturing_command = true
                            }
                        }
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
    println!("name={}", name);
    println!("command={}", command);
    Ok(Item::Topic(Default::default()))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let input = r#"
#       Command: Run unit tests <br>command</br>

```shell @command @cwd=./
echo "About to install ${MIN_VERSION}"
```
    "#;
        let src = MdDocSource::from_str(input)?;
        let items = src.to_items()?;
        dbg!(items);
        Ok(())
    }
}
