use crate::context::Context;
use crate::doc::{Doc, DocResult};
use crate::doc_err::{DocError, Location, LocationError};
use crate::doc_src::DocSrcImpl;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
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

pub struct TomlError<'a> {
    pub doc: &'a Doc,
    pub toml_err: toml::de::Error,
}

impl<'a> From<TomlError<'a>> for DocError {
    fn from(TomlError { toml_err, doc }: TomlError<'a>) -> Self {
        let mut err = LocationError {
            input_file_src: doc.source.content().to_string(),
            location: None,
            input_file: doc.source.file(),
            description: toml_err.to_string(),
        };
        if let Some((line, col)) = toml_err.line_col() {
            err.location = Some(Location::LineAndCol { line, column: col });
            err.description = err.to_string()
        }
        DocError::SerdeLocationErr(err)
    }
}
