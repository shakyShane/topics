use crate::command::Command;
use crate::context::Context;
use crate::dependency::DependencyCheck;
use crate::doc_src::DocSource;
use crate::instruction::Instruction;
use crate::item::Item;
use crate::topic::Topic;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Doc {
    pub input_file: PathBuf,
    pub sources: Vec<DocSource>,
    pub topics: HashMap<String, Topic>,
    pub instructions: HashMap<String, Instruction>,
    pub dep_checks: HashMap<String, DependencyCheck>,
    pub commands: HashMap<String, Command>,
}

impl Doc {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> anyhow::Result<Self> {
        let doc_src = DocSource::from_path_buf(&pb, ctx)?;
        Self::from_doc_src(&pb, doc_src, &ctx)
    }
    pub fn from_doc_src(
        pb: &PathBuf,
        doc_srcs: Vec<DocSource>,
        _ctx: &Context,
    ) -> anyhow::Result<Self> {
        let mut doc = Doc::default();
        doc.input_file = pb.clone();
        doc.sources = doc_srcs;
        for src in &doc.sources {
            let item: Item = serde_yaml::from_str(&src.content).map_err(|e| {
                if let Some(location) = e.location() {
                    DocError::SerdeYamlErr(LocationError {
                        location: Some(Location {
                            line: location.line() + src.line_start,
                            column: location.column(),
                        }),
                        input_file: doc.input_file.clone(),
                        description: e.to_string(),
                    })
                } else {
                    DocError::SerdeYamlErr(LocationError {
                        location: None,
                        input_file: doc.input_file.clone(),
                        description: e.to_string(),
                    })
                }
            })?;
            match item {
                Item::Command(cmd) => {
                    doc.commands.insert(cmd.name.clone(), cmd.clone());
                }
                Item::FileExistsCheck(_) => {}
                Item::DependencyCheck(dc) => {
                    doc.dep_checks.insert(dc.name.clone(), dc.clone());
                }
                Item::Instruction(inst) => {
                    doc.instructions.insert(inst.name.clone(), inst.clone());
                }
                Item::HostEntriesCheck(_) => {}
                Item::Topic(t) => {
                    doc.topics.insert(t.name.clone(), t.clone());
                }
            };
        }
        Ok(doc)
    }
}

#[derive(Debug, thiserror::Error)]
enum DocError {
    #[error(
        "An error occurred when trying to parse a YAML file\n{}",
        .0
    )]
    SerdeYamlErr(LocationError),
}

#[derive(Debug)]
struct LocationError {
    location: Option<Location>,
    input_file: PathBuf,
    description: String,
}

#[derive(Debug)]
struct Location {
    line: usize,
    column: usize,
}

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "");
        writeln!(f, "    msg: `{}`", self.description);
        writeln!(f, "   file: `{}`", self.input_file.display());
        if let Some(location) = &self.location {
            writeln!(f, "   line: `{}`", location.line);
            writeln!(f, " column: `{}`", location.column);
        }
        Ok(())
    }
}

impl FromStr for Doc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc_srcs = DocSource::parse(s)?;
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
        assert_eq!(doc.sources.len(), 1);
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
        assert_eq!(doc.sources.len(), 2);
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
        let srcs = DocSource::parse(input)?;
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
        let srcs = DocSource::parse(input)?;
        let doc = Doc::from_doc_src(&pb, srcs, &Default::default());
        insta::assert_debug_snapshot!(doc);
        Ok(())
    }
}
