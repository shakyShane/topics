use crate::cli::{SubCommand, SubCommandError, SubCommandResult};
use crate::context::Context;
use crate::doc::Doc;
use crate::print::{Print, PrintKind};
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct GenerateCmd {

}

impl SubCommand for GenerateCmd {
    fn exec(&self, ctx: &Context) -> SubCommandResult<()> {
        println!("GenerateCmd not implemented yet");
        Ok(())
    }
}
