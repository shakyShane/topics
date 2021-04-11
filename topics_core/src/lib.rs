#![allow(clippy::module_inception)]
//!
//! Topics core
//!
use crate::cli::{SubCommand, SubCommandItems, SubCommandResult};
use crate::opt::Opt;

mod cli;
mod context;
pub mod cwd;
mod db;
mod doc;
pub mod doc_src;
mod host;
mod items;
mod print;

pub mod doc_err;
pub mod opt;

/// Run a supported sub-command by using the
/// arguments given to the program at runtime
///
/// ```rust no_run
/// use topics_core::from_cli;
/// std::process::exit(match from_cli() {
///     Ok(_) => 0,
///     Err(_) => 1,
/// });
/// ```
pub fn from_cli() -> SubCommandResult<()> {
    let opts = Opt::from_cli_args();
    log::debug!("{:#?}", opts);
    from_opt(&opts)
}

pub fn from_opt(opts: &Opt) -> SubCommandResult<()> {
    let ctx = context::Context::from_opts(&opts);
    match ctx.opts.cmd.as_ref() {
        Some(cmd) => match cmd {
            SubCommandItems::Print(print) => print.exec(&ctx),
            SubCommandItems::Generate(gen) => gen.exec(&ctx),
            SubCommandItems::Verify(verify) => verify.exec(&ctx),
        },
        None => {
            println!("no command given");
            Ok(())
        }
    }
}
