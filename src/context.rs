use crate::doc::Doc;
use crate::opt::Opt;
use crate::print::Print;
use std::path::PathBuf;

#[derive(Debug)]
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
    pub fn read_docs(&self) -> Vec<anyhow::Result<Doc>> {
        self.opts
            .files
            .iter()
            .map(|pb| Doc::from_path_buf(pb, &self))
            .collect()
    }
    pub fn read_docs_split(&self) -> (Vec<anyhow::Result<Doc>>, Vec<anyhow::Result<Doc>>) {
        self.read_docs().into_iter().partition(|a| a.is_ok())
    }
    pub fn cwd(&self) -> PathBuf {
        self.opts.cwd.0.clone()
    }
}

impl Print for Context {
    fn print(&self, doc: &Doc, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print(&doc, &ctx)
    }
    fn print_all(&self, docs: &Vec<anyhow::Result<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        self.opts.print_kind.print_all(&docs, &ctx)
    }
}
