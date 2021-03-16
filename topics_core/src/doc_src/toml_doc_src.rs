use crate::context::Context;
use crate::doc::{Doc, DocError, DocResult, Location, LocationError};
use crate::doc_src::DocSrcImpl;

use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct TomlDocSource {
    pub input_file: Option<PathBuf>,
    pub file_content: String,
}

impl DocSrcImpl for TomlDocSource {
    fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocError::PathRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let new_self = Self {
            input_file: Some(pb.clone()),
            file_content: file_str,
        };
        Ok(new_self)
    }
}

impl FromStr for TomlDocSource {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            input_file: None,
            file_content: s.to_string(),
        })
    }
}
