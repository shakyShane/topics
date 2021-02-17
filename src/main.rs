mod print;
mod output;

use anyhow::Result;
use std::env::current_dir;
use structopt::StructOpt;

use crate::print::print_doc;
use std::path::PathBuf;
use bat::{PrettyPrinter, Input};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    cwd: Option<PathBuf>,

    #[structopt(short, long)]
    index: Option<usize>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    // std::env::set_var("RUST_LOG", "topics=trace");
    env_logger::init();
    let mut opts: Opt = Opt::from_args();
    if opts.files.is_empty() {
        eprintln!("no files provided");
        std::process::exit(1);
    }
    if opts.cwd.is_none() {
        opts.cwd = Some(current_dir().expect("can see current"))
    }
    log::debug!("{:#?}", opts);
    let _ = from_opt(opts)?;
    Ok(())
}

fn from_opt(opt: Opt) -> Result<()> {
    if let Some(cwd) = &opt.cwd {
        for argument in &opt.files {
            let file = std::fs::read_to_string(cwd.join(argument))?;
            let d = serde_yaml::from_str(&file)?;
            let output = print_doc(d, opt.index)?;
            PrettyPrinter::new()
                .header(true)
                // .grid(true)
                // .line_numbers(true)
                .inputs(vec![
                    Input::from_bytes(output.body.as_bytes())
                        .name("topics.md") // Dummy name provided to detect the syntax.
                        .kind("File")
                        .title(output.title),
                ])
                .print()
                .unwrap();
        }
    }
    Ok(())
}
