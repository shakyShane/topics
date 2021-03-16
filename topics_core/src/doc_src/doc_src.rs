use crate::context::Context;
use crate::doc::DocResult;
use crate::doc_src::YamlDocSource;
use std::path::PathBuf;

pub trait DocSrcImpl: Sized {
    fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self>;
}

#[derive(Debug)]
pub enum DocSource {
    Yaml(YamlDocSource),
}
