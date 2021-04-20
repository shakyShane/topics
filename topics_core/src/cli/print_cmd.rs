use crate::cli::{SubCommand, SubCommandError, SubCommandResult};
use crate::context::Context;
use crate::db::try_from_docs;
use crate::doc::Doc;
use crate::html_template::HtmlTemplate;
use crate::print::{OutputKind, Print};
use crate::Outputs;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct PrintCmd {
    #[structopt(short, long, default_value)]
    pub print_kind: OutputKind,

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

        let outputs = try_from_docs(&docs, &self.print_kind).expect("try_from_docs");
        match outputs {
            Outputs::Json(json_output) => {
                let json =
                    serde_json::to_string_pretty(&json_output).expect("serde_json::to_string");
                println!("{}", json);
            }
            Outputs::Html(html_output) => {
                let html_output_dir = ctx.opts.cwd.join("__generated__");
                for p in &html_output.pages {
                    let page_path = html_output_dir.join(&p.pb);
                    let page_str = p.template(&ctx);
                    match page_str {
                        Ok(string) => match fs::write(&page_path, string) {
                            Ok(f) => println!("file written... {}", page_path.display()),
                            Err(e) => {
                                eprintln!("Couldn't write file");
                                eprintln!("{}", e.to_string());
                            }
                        },
                        Err(e) => {
                            eprintln!("{}", e.to_string())
                        }
                    }
                }
                for p in &html_output.assets {
                    let asset_path = html_output_dir.join(&p.pb);
                    let parent = asset_path.parent().expect("must have file parent");
                    let fs_job = fs::create_dir_all(parent).and_then(|()| {
                        fs::write(
                            &asset_path,
                            p.content.as_ref().expect("asset must exist here"),
                        )
                    });

                    match fs_job {
                        Ok(f) => println!("file written... {}", asset_path.display()),
                        Err(e) => {
                            eprintln!("Couldn't write file");
                            eprintln!("{}", e.to_string());
                        }
                    }
                }
            }
        }

        // if let Err(e) = db {
        //     let _ = self.print_kind.print_error(&e.to_string(), &ctx);
        //     return Err(SubCommandError::Handled);
        // }
        //
        // let db = db.expect("previously guarded");
        //
        // if self.all {
        //     let _ = self.print_kind.print_all(&docs, &db, &ctx);
        //     return Ok(());
        // }
        //
        // // if we get here, files were valid, but NO topics were selected
        // let _ = self.print_kind.print_welcome(&docs, &ctx);
        //
        // let titles = docs
        //     .iter()
        //     .map(|doc| doc.topic_names())
        //     .flatten()
        //     .collect::<Vec<&str>>();
        //
        // use dialoguer::MultiSelect;
        //
        // // ctx.print_welcome(&good, &ctx);
        // let selections = MultiSelect::new()
        //     .items(&titles)
        //     .interact()
        //     .map_err(|_| SubCommandError::Unknown)?;
        //
        // for title in selections.iter().map(|idx| titles[*idx]) {
        //     for doc in &docs {
        //         if let Some(topic) = doc.topic_by_name(&title) {
        //             let _ = self.print_kind.print_topic(&topic, &db, &doc, &ctx);
        //         }
        //     }
        // }
        //
        Ok(())
    }
}
