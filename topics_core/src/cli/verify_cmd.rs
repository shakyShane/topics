use crate::cli::{SubCommand, SubCommandError, SubCommandResult};
use crate::context::Context;


use crate::print::{Print, PrintKind};
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
#[structopt(alias = "g")]
pub struct VerifyCmd {
    #[structopt(short, long, default_value)]
    pub print_kind: PrintKind,

    #[structopt(name = "files")]
    files: Vec<PathBuf>,
}

impl SubCommand for VerifyCmd {
    fn exec(&self, ctx: &Context) -> SubCommandResult<()> {
        let (good, bad) = ctx.read_docs_split(&self.files);
        if !bad.is_empty() {
            let _ = self.print_kind.print_errors(&bad, &ctx);
            return Err(SubCommandError::Unknown);
        }
        println!("good={}", good.len());
        println!("bad={}", bad.len());
        // dbg!(bad);
        // dbg!(good);
        Ok(())
    }
}
