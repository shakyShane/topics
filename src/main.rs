mod command;
mod context;
mod cwd;
mod dependency;
mod doc;
mod doc_src;
mod file_exists;
mod host;
mod instruction;
mod item;
mod opt;
mod output;
mod print;
mod topic;

use crate::context::Context;

use crate::opt::Opt;
use crate::print::Print;
use anyhow::Result;

fn main() -> Result<()> {
    // std::env::set_var("RUST_LOG", "topics=trace");
    env_logger::init();
    let opts = Opt::from_cli_args();
    if opts.files.is_empty() {
        eprintln!("no files provided");
        std::process::exit(1);
    }
    log::debug!("{:#?}", opts);
    let ctx = context::Context::from_opts(&opts);
    std::process::exit(match from_opt(&ctx) {
        Ok(_) => 0,
        Err(_) => 1,
    });
}

fn from_opt(ctx: &Context) -> Result<()> {
    let (good, bad) = ctx.read_docs_split();
    if !bad.is_empty() {
        eprintln!("Could not read all documents, please see the info below");
        for item in bad {
            if let Err(e) = item {
                eprintln!("{}", e);
            }
        }
        return Err(anyhow::anyhow!("failed, see above"));
    }
    ctx.print_all(&good, &ctx)
}

#[derive(thiserror::Error, Debug)]
pub enum MainError {
    #[error("print kind not recognised")]
    InvalidFiles { errors: Vec<anyhow::Error> },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        let args = vec![
            "topics",
            "fixtures2/topics.yaml",
            "fixtures2/topics-03.yaml",
        ];
        let ctx = Context::from_vec(&args);
        let (good, bad) = ctx.read_docs_split();
        assert_eq!(good.len(), 1);
        assert_eq!(bad.len(), 1);
    }
}
