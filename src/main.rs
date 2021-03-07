use crate::cli::{SubCommand, SubCommandItems, SubCommandResult};
use crate::context::Context;
use crate::opt::Opt;

mod cli;
mod context;
mod cwd;
mod db;
mod doc;
mod doc_src;
mod host;
mod items;
mod opt;
mod print;

fn main() -> anyhow::Result<()> {
    // std::env::set_var("RUST_LOG", "topics=trace");
    env_logger::init();
    let opts = Opt::from_cli_args();
    log::debug!("{:#?}", opts);
    let ctx = context::Context::from_opts(&opts);
    std::process::exit(match from_opt(&ctx) {
        Ok(_) => 0,
        Err(_) => 1,
    });
}

fn from_opt(ctx: &Context) -> SubCommandResult<()> {
    match ctx.opts.cmd.as_ref() {
        Some(cmd) => match cmd {
            SubCommandItems::Print(print) => print.exec(&ctx),
            SubCommandItems::Generate(gen) => gen.exec(&ctx),
        },
        None => {
            println!("no command given");
            Ok(())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MainError {
    #[error("print kind not recognised")]
    InvalidFiles { errors: Vec<anyhow::Error> },
}
