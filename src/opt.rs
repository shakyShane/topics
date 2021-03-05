use crate::print::PrintKind;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cwd::Cwd;

/// A basic example
#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(short, long, default_value)]
    pub cwd: Cwd,

    #[structopt(short, long, default_value)]
    pub print_kind: PrintKind,

    #[structopt(short, long)]
    pub index: Option<usize>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

impl Opt {
    pub fn from_cli_args() -> Self {
        Opt::from_args()
        // let mut opts: Opt = Opt::from_args();
        // if opts.cwd.is_none() {
        //     opts.cwd = Some(current_dir().expect("can see current"))
        // }
        // opts
    }
    pub fn from_vec(input: &[&str]) -> Self {
        Opt::from_iter(input)
        // let mut opts: Opt = Opt::from_iter(input);
        // if opts.cwd.is_none() {
        //     opts.cwd = Some(current_dir().expect("can see current"))
        // }
        // opts
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_print_kind() {
        let opt = Opt::from_vec(&["prog"]);
        assert_eq!(opt.print_kind, PrintKind::Plain)
    }
}
