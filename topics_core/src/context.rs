use crate::doc::{Doc, DocResult};
use crate::opt::Opt;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Context {
    pub opts: Opt,
}

impl Context {
    pub fn from_opts(opt: &Opt) -> Self {
        Self {
            opts: (*opt).clone(),
        }
    }
    #[cfg(test)]
    pub fn from_vec(input: &[&str]) -> Self {
        Self::from_opts(&Opt::from_vec(input))
    }
    pub fn join_path(&self, pb: impl Into<PathBuf>) -> PathBuf {
        self.opts.cwd.join_path(pb)
    }
    pub fn read_docs_split(&self, files: &[PathBuf]) -> (Vec<DocResult<Doc>>, Vec<DocResult<Doc>>) {
        self.read_docs(&files).into_iter().partition(|a| match a {
            Ok(doc) => doc.errors.is_empty(),
            Err(_) => false,
        })
    }
    pub fn read_docs(&self, files: &[PathBuf]) -> Vec<DocResult<Doc>> {
        files
            .iter()
            .map(|pb| Doc::from_path_buf(pb, &self))
            .collect()
    }
    #[cfg(test)]
    pub fn read_docs_unwrapped(&self, files: &[PathBuf]) -> Vec<Doc> {
        files
            .iter()
            .map(|pb| Doc::from_path_buf(pb, &self).expect("read_docs_unwrapped"))
            .collect()
    }
    pub fn _cwd(&self) -> PathBuf {
        self.opts.cwd.0.clone()
    }
}
