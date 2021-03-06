use crate::context::Context;
use crate::doc::{Doc, DocResult, DocError};
use crate::item::ItemWrap;
use crate::print::Print;

#[derive(Debug)]
pub struct PlainPrinter;

impl Print for PlainPrinter {
    fn print(&self, d: &Doc, _ctx: &Context) -> anyhow::Result<()> {
        let topic_len = d.topics.len();
        println!(
            "\n{} Topic{} from `{}`",
            topic_len,
            if topic_len == 1 { "" } else { "s" },
            d.input_file.display()
        );
        for (index, (name, topic)) in d.topics.iter().enumerate() {
            println!("- {}) {}", index, name);
            println!("  - Dependencies:");
            for dep in &topic.deps {
                match dep {
                    ItemWrap::Named(name) => {
                        println!("     - {}", name);
                    }
                    ItemWrap::Item(item) => {
                        println!("     - {}", item.name());
                    }
                }
            }
            println!("  - Steps:");
            for step in &topic.steps {
                match step {
                    ItemWrap::Named(name) => {
                        println!("     - {}", name);
                    }
                    ItemWrap::Item(item) => {
                        println!("     - {}", item.name());
                    }
                }
            }
        }
        Ok(())
    }
    fn print_all(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        println!("Printing {} doc(s) in Plain format", docs.len());
        for doc in docs {
            if let Ok(doc) = doc {
                let _ = self.print(&doc, &ctx);
            } else {
                eprintln!("could not print a document as it had errored")
            }
        }
        Ok(())
    }

    fn print_errors(&self, docs: &Vec<DocResult<Doc>>, _ctx: &Context) ->  anyhow::Result<()> {
        for doc in docs {
            if let Ok(doc) = doc {
                for error in &doc.errors {
                    print_error(&doc, &error);
                }
            } else {
                eprintln!("could not print a document as it had errored")
            }
        }
        Ok(())
    }
}

fn print_error(doc: &Doc, doc_err: &DocError) {
    match doc_err {
        DocError::FileRead { .. } => {

        }
        DocError::SerdeYamlErr(loc) => {
            if let Some(error_loc) = &loc.location {
                eprintln!("{}", loc.input_file.display());
                eprintln!("{:#?}", error_loc);
            }
        }
    }
}