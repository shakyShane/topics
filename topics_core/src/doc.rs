use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use crate::items::item::Item;

use crate::{context::Context, doc_src::DocSource, items::Topic};
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
        let doc_src = DocSource::from_path_buf(&pb, ctx)?;
        Self::from_doc_src(&pb, doc_src, &ctx)
    }
    pub fn from_doc_src(_pb: &PathBuf, doc_src: DocSource, _ctx: &Context) -> DocResult<Self> {
        let mut doc = Doc {
            source: doc_src,
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new("at line (\\d+)").unwrap();
        }
        for src in &doc.source.doc_src_items.items {
            let item: Result<Item, DocError> = serde_yaml::from_str(&src.content).map_err(|e| {
                let mut err = LocationError {
                    input_file_src: doc.source.file_content.clone(),
                    location: Some(Location::Region {
                        line_start: src.line_start + 1,
                        line_end: src.line_end,
                    }),
                    input_file: doc.source.input_file.clone(),
                    description: e.to_string(),
                };
                if let Some(location) = e.location() {
                    let real_line = location.line() + src.line_start;
                    err.location = Some(Location::LineAndCol {
                        line: real_line,
                        column: location.column(),
                        line_start: src.line_start + 1,
                        line_end: src.line_end,
                    });
                    err.description = RE
                        .replace_all(
                            err.description.as_str(),
                            format!("at line {}", real_line).as_str(),
                        )
                        .to_string()
                }
                DocError::SerdeYamlErr(err)
            });

            match item {
                Err(doc_err) => {
                    doc.errors.push(doc_err);
                }
                Ok(item) => {
                    doc.items.push(ItemTracked {
                        item,
                        src_doc: src.clone(),
                        input_file: doc.source.input_file.clone(),
                    });
                }
            };
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

impl FromStr for Doc {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc_srcs = DocSource::from_str(s)?;
        Doc::from_doc_src(&PathBuf::new(), doc_srcs, &Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_doc() -> anyhow::Result<()> {
        let input = r#"
kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:
  - github-checkin
"#;
        let doc = Doc::from_str(input)?;
        assert_eq!(doc.source.doc_src_items.items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_multi_doc() -> anyhow::Result<()> {
        let input = r#"
---
kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:
  - github-checkin
---
---
---
kind: Instruction
instruction: help me!
name: help-me-instruction
---
---
"#;
        let doc = Doc::from_str(input)?;
        assert_eq!(doc.source.doc_src_items.items.len(), 2);
        Ok(())
    }

    #[test]
    fn test_errors_single() -> anyhow::Result<()> {
        let pb = PathBuf::from("/input-yaml.yml");
        let input = r#"
kind: Topic
name: Run screen shot tests
deps
"#;
        let srcs = DocSource::from_str(input)?;
        let doc = Doc::from_doc_src(&pb, srcs, &Default::default());
        insta::assert_debug_snapshot!(doc);
        Ok(())
    }

    #[test]
    fn test_errors_multi() -> anyhow::Result<()> {
        let pb = PathBuf::from("/input-yaml.yml");
        let input = r#"---

kind: DependencyCheck
name: global-node
verify: node -v
url: https://www.nodejs.org

---

kind: DependencyCheck
name: global-yarn
verify: yarn -v
url: https://yarn.sh/legacy

---

kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps
"#;
        let srcs = DocSource::from_str(input)?;
        let doc = Doc::from_doc_src(&pb, srcs, &Default::default());
        insta::assert_debug_snapshot!(doc?.errors);
        Ok(())
    }
}
