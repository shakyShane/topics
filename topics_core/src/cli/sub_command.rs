use crate::cli::{GenerateCmd, PrintCmd};
use crate::context::Context;

pub trait SubCommand {
    fn exec(&self, ctx: &Context) -> SubCommandResult<()>;
}

pub type SubCommandResult<T, E = SubCommandError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum SubCommandError {
    #[error("An error occurred, but was printed elsewhere")]
    Handled,
    #[error("unknown error occurred")]
    Unknown,
    #[error("No documents were loaded")]
    Empty,
}

#[derive(Debug, Clone, structopt::StructOpt)]
pub enum SubCommandItems {
    Print(PrintCmd),
    Generate(GenerateCmd),
}
