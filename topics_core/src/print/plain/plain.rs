use crate::db::Db;
use crate::doc_err::{DocError, Location};
use crate::items::Item;
use crate::{
    context::Context,
    doc::Doc,
    doc::DocResult,
    items::{ItemWrap, Topic},
    print::Print,
};
use bat::line_range::{LineRange, LineRanges};
use bat::Input;
use bat::PrettyPrinter;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PlainPrinter;

impl Print for PlainPrinter {
    fn print_welcome(&self, _docs: &[Doc], _ctx: &Context) -> anyhow::Result<()> {
        plain_print_heading("Topics", "What would you like to do today?");
        println!();
        Ok(())
    }

    fn print_error(&self, msg: &str, _ctx: &Context) -> anyhow::Result<()> {
        print_error_heading("Problems detected", msg);
        Ok(())
    }

    fn print_heading(&self, kind: &str, message: &str) {
        plain_print_heading(kind, message);
    }

    fn print_topic(
        &self,
        topic: &Topic,
        db: &Db,
        _doc: &Doc,
        _ctx: &Context,
    ) -> anyhow::Result<()> {
        let topic_item = Item::Topic(topic.clone());
        print_item_line(&topic_item, db, 2);
        Ok(())
    }

    fn print_all(&self, docs: &[Doc], db: &Db, ctx: &Context) -> anyhow::Result<()> {
        for doc in docs {
            for topic in &doc.topics() {
                let _ = self.print_topic(&topic, &db, &doc, &ctx);
            }
        }
        Ok(())
    }

    fn print_errors(&self, docs: &[DocResult<Doc>], _ctx: &Context) -> anyhow::Result<()> {
        let summary: Vec<(usize, PathBuf)> =
            docs.iter()
                .fold(vec![], |mut acc, doc_result| match doc_result {
                    Ok(doc) => {
                        acc.push((doc.errors.len(), doc.source.file().unwrap_or_default()));
                        acc
                    }
                    Err(e) => match e {
                        DocError::PathRead { pb, .. } => {
                            acc.push((1, pb.clone()));
                            acc
                        }
                        DocError::SerdeLocationErr(loc_err) => {
                            if let Some(pb) = &loc_err.input_file {
                                acc.push((1, pb.clone()));
                            }
                            acc
                        }
                        DocError::Unknown(_e) => {
                            todo!("how to handle this...")
                        }
                        DocError::NotSupported(pb) => {
                            acc.push((1, pb.clone()));
                            acc
                        }
                    },
                });

        print_error_heading("Problems detected", "Please review the following:");
        eprintln!();
        summary.iter().for_each(|(num_errors, file)| {
            use ansi_term::Colour::{Cyan, Red};
            eprintln!(
                "    {} in {}",
                Red.bold().paint(format!(
                    "{} error{}",
                    num_errors,
                    if *num_errors == 1 { "" } else { "s" }
                )),
                Cyan.paint(file.display().to_string())
            )
        });

        for doc_result in docs {
            match doc_result {
                Ok(doc) => {
                    for error in &doc.errors {
                        print_error(&error);
                    }
                }
                Err(e) => {
                    print_error(&e);
                }
            }
        }
        Ok(())
    }
}

fn print_item_line(item: &Item, db: &Db, width: usize) {
    println!(
        "{:width$}- {kind_name}: {name}",
        " ",
        width = width,
        kind_name = item.kind_name(),
        name = item.name()
    );
    match item {
        Item::Command(_cmd) => {}
        Item::FileExistsCheck(_fec) => {}
        Item::DependencyCheck(_dep_check) => {}
        Item::Instruction(_) => {}
        Item::HostEntriesCheck(_) => {}
        Item::Topic(topic) => {
            if !topic.deps.is_empty() {
                println!("{:1$}- Dependencies:", " ", width + 2);
                for item_wrap in &topic.deps {
                    print_item_wrap(&item_wrap, &db, width + 4);
                }
            }
            if !topic.steps.is_empty() {
                println!("{:1$}- Steps:", " ", width + 2);
                for item_wrap in &topic.steps {
                    print_item_wrap(&item_wrap, &db, width + 4);
                }
            }
        }
        Item::TaskGroup(_) => {}
    }
}

fn print_item_wrap(item_wrap: &ItemWrap, db: &Db, width: usize) {
    // dbg!(&db);
    let maybe_item = match item_wrap {
        ItemWrap::Named(name) => (name.clone(), db.item_map.get(name)),
        ItemWrap::Item(item) => {
            let name = item.name();
            (name.clone(), db.item_map.get(&name))
        }
    };
    match maybe_item {
        (_name, Some(matched_item)) => {
            print_item_line(&matched_item.item, &db, width);
        }
        (name, None) => {
            println!(
                "{:width$}- NOT_FOUND: {name}",
                " ",
                width = width,
                name = name
            );
        }
    }
}

fn print_error_heading(kind: &str, message: &str) {
    use ansi_term::Colour::Red;
    use ansi_term::{ANSIString, ANSIStrings};
    eprintln!();
    let some_value = kind.to_string();
    let strings: &[ANSIString<'static>] =
        &[Red.paint("["), Red.bold().paint(some_value), Red.paint("]")];
    eprintln!("{} {}", ANSIStrings(strings), Red.bold().paint(message));
}

fn plain_print_heading(kind: &str, message: &str) {
    use ansi_term::Colour::Green;
    use ansi_term::{ANSIString, ANSIStrings};
    eprintln!();
    let some_value = kind.to_string();
    let strings: &[ANSIString<'static>] = &[
        Green.paint("["),
        Green.bold().paint(some_value),
        Green.paint("]"),
    ];
    eprintln!("{} {}", ANSIStrings(strings), Green.bold().paint(message));
}

fn print_error(doc_err: &DocError) {
    match doc_err {
        DocError::PathRead {
            pb,
            abs: _,
            original,
        } => {
            print_error_heading("File error", &original.to_string());
            match original.kind() {
                ErrorKind::NotFound => {
                    eprintln!();
                    eprintln!("    A given path could not be found, please check your input");
                }
                ErrorKind::Other => {}
                _ => {
                    eprintln!("An unknown error occurred");
                }
            }
            use ansi_term::Colour::Green;
            eprintln!();
            eprintln!("    input: {}", Green.paint(&pb.display().to_string()));
        }
        DocError::Unknown(_) | DocError::NotSupported(_) => {
            print_error_heading("Error", &doc_err.to_string());
        }
        DocError::SerdeLocationErr(loc_err) => {
            let name = loc_err
                .input_file
                .as_ref()
                .map(|f| f.display().to_string())
                .unwrap_or_default();
            print_error_heading("error", &loc_err.description);
            if let Some(error_loc) = &loc_err.location {
                match error_loc {
                    Location::LineAndColRegion {
                        line,
                        line_end,
                        line_start,
                        ..
                    } => {
                        let mut printer = setup();

                        printer.highlight(*line);

                        // regions
                        ranges(&mut printer, *line_start, *line_end);
                        inputs(
                            &mut printer,
                            &name,
                            loc_err.input_file_src.as_bytes(),
                            *line,
                            &loc_err.input_file,
                        );

                        printer.print().unwrap();
                    }
                    Location::Region {
                        line_end,
                        line_start,
                    } => {
                        let mut printer = setup();
                        ranges(&mut printer, *line_start, *line_end);
                        inputs(
                            &mut printer,
                            &name,
                            loc_err.input_file_src.as_bytes(),
                            *line_start,
                            &loc_err.input_file,
                        );
                        printer.print().unwrap();
                    }
                    Location::LineAndCol { line, column: _ } => {
                        let mut printer = setup();
                        printer.highlight(*line);
                        inputs(
                            &mut printer,
                            &name,
                            loc_err.input_file_src.as_bytes(),
                            *line,
                            &loc_err.input_file,
                        );
                        printer.print().unwrap();
                    }
                }
            }
        }
    }
}

fn setup() -> PrettyPrinter<'static> {
    let mut printer = PrettyPrinter::new();
    printer.header(true).line_numbers(true).grid(true);
    printer
}

fn ranges(printer: &mut PrettyPrinter, start: usize, end: usize) {
    printer.line_ranges(LineRanges::from(vec![LineRange::new(start, end)]));
}

fn inputs<'a>(
    printer: &mut PrettyPrinter<'a>,
    name: &'a str,
    bytes: &'a [u8],
    line: usize,
    pb: &'a Option<PathBuf>,
) {
    let filename = pb
        .as_ref()
        .map(|f| f.display().to_string())
        .unwrap_or_default();
    let file_name_heading = if line > 0 {
        format!("{}:{}", filename, line)
    } else {
        filename
    };
    printer.inputs(vec![Input::from_bytes(bytes)
        .name(&name) // Dummy name provided to detect the syntax.
        .kind("File")
        .title(file_name_heading)]);
}
