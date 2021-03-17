use crate::doc_err::DocError;
use crate::doc_src::{from_serde_yaml_error, DocSource, TomlError};
use crate::items::item::Item;
use crate::{context::Context, items::Topic};
use std::path::PathBuf;
use pulldown_cmark::{Parser, Event, Tag, Options, CodeBlockKind};
use std::str::Split;

#[derive(Debug, Default)]
pub struct Doc {
    pub source: DocSource,
    pub items: Vec<ItemTracked>,
    pub errors: Vec<DocError>,
}

#[derive(Debug, Clone)]
pub struct ItemTracked {
    pub item: Item,
    pub src_doc: DocSource,
    pub input_file: Option<PathBuf>,
}

pub type DocResult<T, E = DocError> = core::result::Result<T, E>;

impl Doc {
    pub fn topics(&self) -> Vec<Topic> {
        self.items
            .iter()
            .filter_map(|item| match item {
                ItemTracked {
                    item: Item::Topic(topic),
                    ..
                } => Some(topic.clone()),
                _ => None,
            })
            .collect()
    }
    pub fn topic_names(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter_map(|item| match item {
                ItemTracked {
                    item: Item::Topic(topic),
                    ..
                } => Some(topic.name.as_str()),
                _ => None,
            })
            .collect()
    }
    pub fn topic_by_name(&self, name: &str) -> Option<&Topic> {
        self.items.iter().find_map(|item| match item {
            ItemTracked {
                item: Item::Topic(topic),
                ..
            } => {
                if topic.name == name {
                    Some(topic)
                } else {
                    None
                }
            }
            _ => None,
        })
    }
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let doc_src = match pb.extension() {
            None => todo!("what to handle here?"),
            Some(os_str) => match os_str.to_str() {
                None => todo!("what to handle here?"),
                Some("yaml") | Some("yml") => DocSource::yaml(&pb, ctx)?,
                Some("toml") => DocSource::toml(&pb, ctx)?,
                Some(_other) => return Err(DocError::NotSupported(pb.clone())),
            },
        };
        Self::from_doc_src(&pb, doc_src, &ctx)
    }
    pub fn from_doc_src(_pb: &PathBuf, doc_src: DocSource, _ctx: &Context) -> DocResult<Self> {
        let mut doc = Doc {
            source: doc_src,
            ..Default::default()
        };
        match &doc.source {
            DocSource::Yaml(yaml_doc) => {
                for src in &yaml_doc.doc_src_items.items {
                    let item: Result<Item, DocError> = serde_yaml::from_str(&src.content)
                        .map_err(|err| from_serde_yaml_error(&doc, &src, &err));
                    match item {
                        Err(doc_err) => {
                            doc.errors.push(doc_err);
                        }
                        Ok(item) => {
                            doc.items.push(ItemTracked {
                                item,
                                src_doc: DocSource::Yaml((*yaml_doc).clone()),
                                input_file: yaml_doc.input_file.clone(),
                            });
                        }
                    };
                }
            }
            DocSource::Toml(toml_doc) => {
                let items = one_or_many_toml(&toml_doc.file_content).map_err(|err| TomlError {
                    doc: &doc,
                    toml_err: err,
                })?;
                for item in items {
                    doc.items.push(ItemTracked {
                        item,
                        src_doc: DocSource::Toml((*toml_doc).clone()),
                        input_file: toml_doc.input_file.clone(),
                    });
                }
            }
        }
        Ok(doc)
    }
}

fn one_or_many_toml(input: &str) -> Result<Vec<Item>, toml::de::Error> {
    #[derive(Debug, serde::Deserialize)]
    struct TempItems {
        item: Vec<Item>,
    }
    toml::from_str::<TempItems>(input)
        .or_else(|err| {
            if err
                .to_string()
                .contains("missing field `item` at line 1 column 1")
            {
                toml::from_str::<Item>(input).map(|item| TempItems { item: vec![item] })
            } else {
                Err(err)
            }
        })
        .map(|temp| temp.item)
}

#[test]
fn test_toml() -> anyhow::Result<()> {
    let input = r#"
[[item]]
kind = "Topic"
name = "Run unit tests"
steps = [
    "Install root-level dependencies",
    "Run unit tests command"
]
deps = [
    "install nodejs"
]

[[item]]
kind = "Command"
name = "Run unit tests command"
cwd = "."
command = """
echo hello world!
"""

[[item]]
kind = "Command"
name = "Install root-level dependencies"
cwd = "."
command = """
echo hello world!
"""

[[item]]
kind = "DependencyCheck"
name = "install nodejs"
verify = """
node -v
"""
    "#;
    #[derive(Debug, serde::Deserialize)]
    struct Items {
        item: Vec<Item>,
    }
    let items: Items = toml::from_str(input)?;
    assert_eq!(items.item.len(), 4);
    Ok(())
}

#[test]
fn test_toml_single() {
    let input = r#"
kind = "Command"
cwd = "."
name = "Run unit tests command"
command = """
echo "About to install ${MIN_VERSION}"
"""

env.HELLO = { from = "env vars", key = "minimum_skaffold_version" }

    "#;
    let items = one_or_many_toml(input);
    println!("{:?}", items);
}

#[test]
fn test_md_single() {
    let input = r#"
#       Command: Run unit tests <br>command</br>

```shell @command @cwd=./
echo "About to install ${MIN_VERSION}"
```

some random `stuff`

    "#;
    let mut kind = "";
    let mut name = "".to_string();
    let mut collecting_name = false;

    let mut command = "".to_string();
    let mut capturing_command = false;
    let mut cwd: Option<String> = None;

    for evt in Parser::new_ext(input, Options::empty()) {
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
                    Tag::Heading(1) => {
                        collecting_name = false
                    }
                    Tag::CodeBlock(code) => match code {
                        CodeBlockKind::Indented => {}
                        CodeBlockKind::Fenced(fence_args) => {
                            capturing_command = false
                        }
                    }
                    _ => {
                        // noop
                    }
                }
            }
            Event::Start(a) => {
                match a {
                    Tag::Paragraph => {}
                    Tag::Heading(1) => {
                        if !collecting_name && name.is_empty() {
                            collecting_name = true
                        }
                    }
                    Tag::Heading(_other) => {}
                    Tag::BlockQuote => {}
                    Tag::CodeBlock(code) => {
                        match code {
                            CodeBlockKind::Indented => {
                                println!("code indented");
                            }
                            CodeBlockKind::Fenced(fence_args) => {
                                if !capturing_command && command.is_empty() {
                                    if fence_args.contains("@command") {
                                        fence_args.split_whitespace()
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
                                                    },
                                                    (Some("@cwd"), None) => {
                                                        println!("MISSING CWD VALUE");
                                                    },
                                                    _ => println!("other")
                                                }
                                            });
                                        capturing_command = true
                                    }
                                }
                            }
                        }
                    }
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
                }
            }
            _ => {
                // println!("other")
            }
        }
    }
    println!("name={}", name);
    println!("command={}", command);
}
