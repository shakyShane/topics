use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::items::item::Item;

use crate::doc_src::{from_serde_yaml_error, DocSource};
use crate::{context::Context, items::Topic};
use multi_yaml::YamlDoc;

#[derive(Debug, Default)]
pub struct Doc {
    pub source: DocSource,
    pub items: Vec<ItemTracked>,
    pub errors: Vec<DocError>,
}

#[derive(Debug, Clone)]
pub struct ItemTracked {
    pub item: Item,
    pub src_doc: YamlDoc,
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
                                src_doc: src.clone(),
                                input_file: yaml_doc.input_file.clone(),
                            });
                        }
                    };
                }
            }
            DocSource::Toml(_toml_doc) => {
                println!("got toml!");
            }
        }
        Ok(doc)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error(
    "could not read file `{}`\nFull path: {}",
    pb.display(),
    abs.display()
    )]
    PathRead {
        pb: PathBuf,
        abs: PathBuf,
        original: std::io::Error,
    },
    #[error(
        "{}",
        .0
    )]
    SerdeYamlErr(LocationError),
    #[error("{}", .0)]
    Unknown(String),
    #[error("File format not supported: {}", .0.display())]
    NotSupported(PathBuf),
}

impl From<anyhow::Error> for DocError {
    fn from(e: anyhow::Error) -> Self {
        DocError::Unknown(e.to_string())
    }
}

#[derive(Debug)]
pub struct LocationError {
    pub location: Option<Location>,
    pub input_file: Option<PathBuf>,
    pub input_file_src: String,
    pub description: String,
}

#[derive(Debug)]
pub enum Location {
    LineAndCol {
        line_start: usize,
        line_end: usize,
        line: usize,
        column: usize,
    },
    Region {
        line_start: usize,
        line_end: usize,
    },
    // Unknown,
}

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f);
        if let Some(location) = &self.location {
            match location {
                Location::LineAndCol { line, column, .. } => {
                    let _ = writeln!(f, "    msg: {}", self.description);
                    let _ = writeln!(
                        f,
                        "   file: {}",
                        self.input_file
                            .as_ref()
                            .map(|f| f.display().to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    let _ = writeln!(f, "   line: {}", line);
                    let _ = writeln!(f, " column: {}", column);
                }
                Location::Region {
                    line_start,
                    line_end,
                } => {
                    let _ = writeln!(f, "           msg: {}", self.description);
                    let _ = writeln!(
                        f,
                        "          file: {}",
                        self.input_file
                            .as_ref()
                            .map(|f| f.display().to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    let _ = writeln!(f, " between lines: {} & {}", line_start, line_end);
                }
            }
        }
        Ok(())
    }
}
