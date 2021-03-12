use crate::cli::{SubCommand, SubCommandError, SubCommandResult};
use crate::context::Context;
use crate::db::Db;
use crate::doc::Doc;
use crate::print::{Print, PrintKind};
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct PrintCmd {
    #[structopt(short, long, default_value)]
    pub print_kind: PrintKind,

    #[structopt(short, long)]
    pub index: Option<usize>,

    #[structopt(short, long)]
    pub all: bool,

    /// Files to process, you should likely include everything here
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
        if good.is_empty() {
            let err = SubCommandError::Empty;
            let _ = self.print_kind.print_error(&err.to_string(), &ctx);
            return Err(err);
        }

        let docs = good
            .into_iter()
            .map(|doc| doc.expect("guarded previously"))
            .collect::<Vec<Doc>>();

        let db = Db::try_from_docs(&docs);

        if let Err(e) = db {
            let _ = self.print_kind.print_error(&e.to_string(), &ctx);
            return Err(SubCommandError::Handled);
        }

        let db = db.expect("previously guarded");

        if self.all {
            let _ = self.print_kind.print_all(&docs, &db, &ctx);
            return Ok(());
        }

        // if we get here, files were valid, but NO topics were selected
        let _ = self.print_kind.print_welcome(&docs, &ctx);

        let titles = docs
            .iter()
            .map(|doc| doc.topics.keys())
            .flatten()
            .collect::<Vec<&String>>();

        use dialoguer::MultiSelect;

        // ctx.print_welcome(&good, &ctx);
        let selections = MultiSelect::new()
            .items(&titles)
            .interact()
            .map_err(|_| SubCommandError::Unknown)?;

        // let _titles = selections
        //     .into_iter()
        //     .map(|idx| titles[idx])
        //     .map(|title| docs.iter().filter_map(|doc| doc.topics.clone().get(title)));

        // dbg!(_titles);

        // let matched = selections.iter().map(|selection| {
        //     let name = titles[*selection];
        //     let clone = name.clone();
        //     let matched = docs.iter().filter_map(|doc| doc.topics.get(&clone));
        //     matched
        // });

        // dbg!(selections);
        for title in selections.iter().map(|idx| titles[*idx]) {
            for doc in &docs {
                if let Some(topic) = doc.topics.get(title) {
                    let _ = self.print_kind.print_topic(&topic, &db, &doc, &ctx);
                }
            }
        }

        Ok(())
    }
}
