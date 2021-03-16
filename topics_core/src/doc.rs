use crate::doc_err::DocError;
use crate::doc_src::{from_serde_yaml_error, DocSource, TomlError};
use crate::items::item::Item;
use crate::{context::Context, items::Topic};
use std::path::PathBuf;

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
                #[derive(Debug, serde::Deserialize)]
                struct TempItems {
                    item: Vec<Item>,
                }
                let items: TempItems =
                    toml::from_str(&toml_doc.file_content).map_err(|err| TomlError {
                        doc: &doc,
                        toml_err: err,
                    })?;
                for item in items.item {
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
