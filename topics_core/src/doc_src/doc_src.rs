use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_src::{TomlDocSource, YamlDocSource};
use std::path::PathBuf;

pub trait DocSrcImpl: Sized {
    fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self>;
}

#[derive(Debug)]
pub enum DocSource {
    Yaml(YamlDocSource),
    Toml(TomlDocSource),
}

impl DocSource {
    pub fn file(&self) -> Option<PathBuf> {
        match self {
            DocSource::Yaml(yaml_doc) => yaml_doc.input_file.clone(),
            DocSource::Toml(toml_doc) => toml_doc.input_file.clone(),
        }
    }
    pub fn content(&self) -> &str {
        match self {
            DocSource::Yaml(yaml_doc) => yaml_doc.file_content.as_str(),
            DocSource::Toml(toml_doc) => toml_doc.file_content.as_str(),
        }
    }
    pub fn yaml(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        Ok(DocSource::Yaml(YamlDocSource::from_path_buf(&pb, ctx)?))
    }
    pub fn toml(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        Ok(DocSource::Toml(TomlDocSource::from_path_buf(&pb, ctx)?))
    }
}

impl Default for DocSource {
    fn default() -> Self {
        Self::Yaml(YamlDocSource::default())
    }
}
