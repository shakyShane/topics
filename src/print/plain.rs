use crate::context::Context;
use crate::doc::{Doc, DocError, DocResult, Location};
use crate::item::ItemWrap;
use crate::print::Print;
use bat::line_range::{LineRange, LineRanges};
use bat::{Input, PrettyPrinter};

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

    fn print_errors(&self, docs: &Vec<DocResult<Doc>>, _ctx: &Context) -> anyhow::Result<()> {
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
            eprintln!("{}", doc_err);
        }
        DocError::SerdeYamlErr(loc_err) => {
            use ansi_term::Colour::Red;
            use ansi_term::{ANSIString, ANSIStrings};
            eprint!("\n");
            let some_value = format!("{}", "YAML error");
            let strings: &[ANSIString<'static>] =
                &[Red.paint("["), Red.bold().paint(some_value), Red.paint("]")];
            eprintln!(
                "{} {}",
                ANSIStrings(strings),
                Red.bold().paint(&loc_err.description)
            );
            if let Some(error_loc) = &loc_err.location {
                match error_loc {
                    Location::LineAndCol { line, .. } => {
                        PrettyPrinter::new()
                            .header(true)
                            .line_numbers(true)
                            .grid(true)
                            .highlight(*line)
                            .inputs(vec![Input::from_bytes(doc.source.file_content.as_bytes())
                                .name(&doc.input_file) // Dummy name provided to detect the syntax.
                                .kind("File")
                                .title(&loc_err.input_file.display().to_string())])
                            .print()
                            .unwrap();
                    }
                    Location::Region {
                        line_end,
                        line_start,
                    } => {
                        PrettyPrinter::new()
                            .header(true)
                            .line_numbers(true)
                            .grid(true)
                            .line_ranges(LineRanges::from(vec![LineRange::new(
                                *line_start,
                                *line_end,
                            )]))
                            .inputs(vec![Input::from_bytes(doc.source.file_content.as_bytes())
                                .name(&doc.input_file) // Dummy name provided to detect the syntax.
                                .kind("File")
                                .title(&loc_err.input_file.display().to_string())])
                            .print()
                            .unwrap();
                    }
                    Location::Unknown => {}
                }
            }
        }
    }
}
