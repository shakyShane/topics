use crate::cli::{SubCommand, SubCommandError, SubCommandResult};
use crate::context::Context;
use crate::print::{Print, PrintKind};
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct PrintCmd {
    #[structopt(short, long, default_value)]
    pub print_kind: PrintKind,

    #[structopt(short, long)]
    pub index: Option<usize>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

impl SubCommand for PrintCmd {
    fn exec(&self, ctx: &Context) -> SubCommandResult<()> {
        let (good, bad) = ctx.read_docs_split(&self.files);
        if !bad.is_empty() {
            let _ = self.print_kind.print_errors(&bad, &ctx);
            return Err(SubCommandError::Unknown);
        }
        let titles = good
            .iter()
            .filter_map(|item| item.as_ref().ok().map(|doc| doc.topics.keys()))
            .flatten()
            .collect::<Vec<&String>>();
        use dialoguer::MultiSelect;

        // ctx.print_welcome(&good, &ctx);
        let selection = MultiSelect::new()
            .items(&titles)
            .interact()
            .map_err(|_| SubCommandError::Unknown)?;
        println!(
            "selected = {:?}",
            selection
                .iter()
                .map(|idx| titles[*idx])
                .collect::<Vec<&String>>()
        );
        Ok(())
    }
}
