use crate::doc::{Doc, DocResult};
use crate::opt::Opt;
use crate::print::Print;
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
    pub fn from_vec(input: &[&str]) -> Self {
        Self::from_opts(&Opt::from_vec(input))
    }
    pub fn join_path(&self, pb: impl Into<PathBuf>) -> PathBuf {
        self.opts.cwd.join_path(pb)
    }
    pub fn read_docs_split(&self) -> (Vec<DocResult<Doc>>, Vec<DocResult<Doc>>) {
        self.read_docs().into_iter().partition(|a| match a {
            Ok(doc) => doc.errors.is_empty(),
            Err(_) => false,
        })
    }
    pub fn read_docs(&self) -> Vec<DocResult<Doc>> {
        self.opts
            .files
            .iter()
            .map(|pb| Doc::from_path_buf(pb, &self))
            .collect()
    }
    pub fn cwd(&self) -> PathBuf {
        self.opts.cwd.0.clone()
    }
}

impl Print for Context {
    fn print(&self, doc: &Doc, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print(&doc, &ctx)
    }

    fn print_welcome(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print_welcome(&docs, &ctx)
    }

    fn print_all(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print_all(&docs, &ctx)
    }
    fn print_errors(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print_errors(&docs, &ctx)
    }
}
