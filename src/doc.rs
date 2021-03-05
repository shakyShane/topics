use crate::command::Command;
use crate::context::Context;
use crate::dependency::DependencyCheck;
use crate::instruction::Instruction;
use crate::item::Item;
use crate::topic::Topic;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Doc {
    pub source: Option<DocSource>,
    pub topics: HashMap<String, Topic>,
    pub instructions: HashMap<String, Instruction>,
    pub dep_checks: HashMap<String, DependencyCheck>,
    pub commands: HashMap<String, Command>,
}

impl Doc {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> anyhow::Result<Self> {
        let source = DocSource {
            original: pb.clone(),
            absolute: ctx.join_path(pb),
            cwd: ctx.cwd(),
        };
        let attempt = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&attempt).map_err(|e| DocError::FileRead {
            pb: pb.clone(),
            abs: attempt,
            original: e,
        })?;
        Self::from_str_doc(&file_str, &ctx, Some(source))
    }
    pub fn from_str_doc(str: &str, _ctx: &Context, src: Option<DocSource>) -> anyhow::Result<Self> {
        let mut doc = Doc::default();
        doc.source = src;
        for document in serde_yaml::Deserializer::from_str(&str) {
            let value = Item::deserialize(document);
            if let Err(e) = value {
                eprintln!("e={}", e);
                return Err(e.into());
            }
            match value? {
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

#[derive(Debug, serde::Deserialize)]
pub struct DocSource {
    pub original: PathBuf,
    pub absolute: PathBuf,
    pub cwd: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum DocError {
    #[error(
        "FileRead error: could not read file `{}`\nFull path: {}",
        pb.display(),
        abs.display()
    )]
    FileRead {
        pb: PathBuf,
        abs: PathBuf,
        original: std::io::Error,
    },
}

#[cfg(test)]
mod test {

    use crate::doc::Doc;
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_deserialise() -> anyhow::Result<()> {
        let ctx = Context::from_vec(&[]);
        let pb = current_dir()?.join("fixtures2/topics.yaml");
        let input = r#"kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:

"#;
        let s = input.split("---");
        let i = serde_yaml::from_str::<Item>(input);
        if let Err(e) = i {
            println!("{}", e);
        }
        // dbg!(t);
        Ok(())
    }
}
