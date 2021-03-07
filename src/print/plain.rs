use crate::{
    doc::DocResult,
    doc::DocError,
    doc::Doc,
    context::Context,
    doc::Location,
    items::{Topic, ItemWrap},
    print::Print
};
use bat::line_range::{LineRange, LineRanges};
use bat::Input;
use bat::PrettyPrinter;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PlainPrinter;

impl Print for PlainPrinter {
    fn print(&self, doc: &Doc, ctx: &Context) -> anyhow::Result<()> {
        let topic_len = doc.topics.len();
        println!(
            "\n{} Topic{} from `{}`",
            topic_len,
            if topic_len == 1 { "" } else { "s" },
            doc.input_file.display()
        );
        for (_index, (_name, topic)) in doc.topics.iter().enumerate() {
            let _ = self.print_heading("Topic", &topic.name);
            let _ = self.print_topic(&topic, &doc, &ctx);
        }
        Ok(())
    }

    fn print_welcome(&self, _docs: &Vec<Doc>, _ctx: &Context) -> anyhow::Result<()> {
        plain_print_heading("Topics", "What would you like to do today?");
        println!();
        Ok(())
    }

    fn print_heading(&self, kind: &str, message: &str) {
        plain_print_heading(kind, message);
    }

    fn print_topic(&self, topic: &Topic, _doc: &Doc, _ctx: &Context) -> anyhow::Result<()> {
        println!();
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
        let summary: Vec<(usize, PathBuf)> =
            docs.iter()
                .fold(vec![], |mut acc, doc_result| match doc_result {
                    Ok(doc) => {
                        acc.push((doc.errors.len(), doc.input_file.clone()));
                        acc
                    }
                    Err(e) => match e {
                        DocError::PathRead { pb, .. } => {
                            acc.push((1, pb.clone()));
                            acc
                        }
                        DocError::SerdeYamlErr(_) => unreachable!("shouldn't get here"),
                    },
                });

        print_error_heading("Problems detected", "Please review the following:");
        eprintln!();
        summary.iter().for_each(|(num, file)| {
            use ansi_term::Colour::{Cyan, Red};
            eprintln!(
                "    {} in {}",
                Red.bold()
                    .paint(format!("{} error{}", num, if *num == 1 { "" } else { "s" })),
                Cyan.paint(file.display().to_string())
            )
        });

        for doc_result in docs {
            match doc_result {
                Ok(doc) => {
                    // eprintln!("{} errors found in {}", doc.input_file.display());

                    for error in &doc.errors {
                        print_error(&doc, &error);
                    }
                }
                Err(e) => {
                    print_doc_error(&e);
                }
            }
        }
        Ok(())
    }
}

fn print_doc_error(doc_err: &DocError) {
    match doc_err {
        DocError::PathRead {
            original,
            abs: _,
            pb,
        } => {
            print_error_heading("File error", &original.to_string());
            match original.kind() {
                ErrorKind::NotFound => {
                    eprintln!();
                    eprintln!("    A given path could not be found, please check your input");
                }
                ErrorKind::Other => {}
                _ => {
                    eprintln!("An unknown error occured");
                }
            }
            use ansi_term::Colour::Green;
            eprintln!();
            eprintln!("    input: {}", Green.paint(&pb.display().to_string()));
        }
        _ => unimplemented!("use print error for this"),
    }
}

fn print_error_heading(kind: &str, message: &str) {
    use ansi_term::Colour::Red;
    use ansi_term::{ANSIString, ANSIStrings};
    eprint!("\n");
    let some_value = format!("{}", kind);
    let strings: &[ANSIString<'static>] =
        &[Red.paint("["), Red.bold().paint(some_value), Red.paint("]")];
    eprintln!("{} {}", ANSIStrings(strings), Red.bold().paint(message));
}

fn plain_print_heading(kind: &str, message: &str) {
    use ansi_term::Colour::Green;
    use ansi_term::{ANSIString, ANSIStrings};
    eprint!("\n");
    let some_value = format!("{}", kind);
    let strings: &[ANSIString<'static>] = &[
        Green.paint("["),
        Green.bold().paint(some_value),
        Green.paint("]"),
    ];
    eprintln!("{} {}", ANSIStrings(strings), Green.bold().paint(message));
}

fn print_error(doc: &Doc, doc_err: &DocError) {
    match doc_err {
        DocError::PathRead {
            pb: _,
            abs: _,
            original: _,
        } => {
            // eprintln!("{}", doc_err);
            // println!("{}", original.to_string());
        }
        DocError::SerdeYamlErr(loc_err) => {
            print_error_heading("YAML error", &loc_err.description);
            if let Some(error_loc) = &loc_err.location {
                match error_loc {
                    Location::LineAndCol {
                        line,
                        line_end,
                        line_start,
                        ..
                    } => {
                        PrettyPrinter::new()
                            .header(true)
                            .line_numbers(true)
                            .grid(true)
                            .highlight(*line)
                            .line_ranges(LineRanges::from(vec![LineRange::new(
                                *line_start,
                                *line_end,
                            )]))
                            .inputs(vec![Input::from_bytes(doc.source.file_content.as_bytes())
                                .name(&doc.input_file) // Dummy name provided to detect the syntax.
                                .kind("File")
                                .title(format!(
                                    "{}:{}",
                                    &loc_err.input_file.display().to_string(),
                                    line
                                ))])
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
                                .title(format!(
                                    "{}:{}",
                                    &loc_err.input_file.display().to_string(),
                                    line_start
                                ))])
                            .print()
                            .unwrap();
                    }
                }
            }
        }
    }
}
