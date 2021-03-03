use crate::context::Context;
use crate::step::Step;
use crate::topic::Topic;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct Doc {
    pub source: Option<DocSource>,
    pub topics: Vec<Topic>,
    pub commands: Option<Vec<Step>>,
    pub steps: Option<Vec<Step>>,
    pub multi_steps: Option<Vec<Step>>,
}

impl Doc {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> anyhow::Result<Self> {
        let source = DocSource {
            original: pb.clone(),
            absolute: ctx.join_path(pb),
            cwd: ctx.cwd(),
        };
        let attempt = ctx.join_path(pb);
        let file = std::fs::read_to_string(&attempt).map_err(|e| DocError::FileRead {
            pb: pb.clone(),
            abs: attempt,
            original: e,
        })?;
        let mut doc: Doc = serde_yaml::from_str(&file)?;
        doc.source = Some(source);
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
