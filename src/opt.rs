use structopt::StructOpt;

use crate::cli::SubCommandItems;
use crate::cwd::Cwd;

/// A basic example
#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(short, long, default_value)]
    pub cwd: Cwd,

    #[structopt(subcommand)]
    pub cmd: Option<SubCommandItems>,
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
