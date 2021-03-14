use crate::cli::{SubCommand, SubCommandResult};
use crate::context::Context;

use crate::items::{DependencyCheck, Item};
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
#[structopt(alias = "g")]
pub struct VerifyCmd {
    #[structopt(name = "items")]
    items: Vec<PathBuf>,
}

impl SubCommand for VerifyCmd {
    fn exec(&self, _ctx: &Context) -> SubCommandResult<()> {
        println!("self={:?}", self);
        Ok(())
    }
}
