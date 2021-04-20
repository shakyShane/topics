use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

use crate::db_error::{CycleError, DbError, ErrorRef, IntoDbError, SerializedError};
use crate::doc::Doc;
use crate::doc_src::{DocSource, MdDocSource, MdSrc};
use crate::html::output_html;
use crate::items::{marker_ref, name_ref, Item, ItemWrap, LineMarker};
use crate::output::Output;
use crate::print::OutputKind;
use crate::Outputs;

#[derive(Debug)]
pub struct Db {}

pub fn try_from_docs(docs: &[Doc], output_kind: &OutputKind) -> anyhow::Result<Outputs> {
    let mut src_items: Vec<MdSrc> = vec![];

    for doc in docs {
        if let DocSource::Md(md) = &doc.source {
            for item in &md.doc_src_items.items {
                let next = MdSrc::new(md, item);
                src_items.push(next);
            }
        }
    }

    for item in src_items.iter() {
        item.parse();
    }

    let mut graph: HashMap<&'_ String, Vec<&'_ LineMarker<String>>> = HashMap::new();

    let items: Vec<(&'_ MdSrc, Vec<Item>)> =
        src_items.iter().map(|src| (src, src.as_items())).collect();

    let mut item_lookup: HashMap<&'_ String, (&'_ MdSrc, &'_ Item)> = HashMap::new();

    for (mdsrc, items) in &items {
        for item in items {
            item_lookup.insert(name_ref(item), (mdsrc, item));
        }
    }

    for (_mdsrc, items) in &items {
        for item in items {
            let lm = marker_ref(item);
            let entry = graph.entry(&lm.item).or_insert(Vec::new());
            if let Item::Topic(topic) = item {
                for named_ref in topic.deps.iter().chain(topic.steps.iter()) {
                    match named_ref {
                        ItemWrap::NamedRef(line_marker) => {
                            entry.push(line_marker);
                        }
                        ItemWrap::Item(_) => todo!("inline item"),
                    }
                }
            }
        }
    }

    match output_kind {
        OutputKind::Plain => todo!("plain"),
        OutputKind::Markdown => todo!("markdown"),
        OutputKind::Json => Ok(Outputs::Json(output_json(&graph, &item_lookup, &items))),
        OutputKind::Html => Ok(Outputs::Html(output_html(&graph, &item_lookup, &items))),
    }
}

fn output_json<'a>(
    graph: &'a HashMap<&'a String, Vec<&'a LineMarker<String>>>,
    lookup: &'a HashMap<&'a String, (&'a MdSrc<'a>, &'a Item)>,
    items: &'a Vec<(&'_ MdSrc, Vec<Item>)>,
) -> Output {
    let mut output = Output::default();
    let cycles = detect_cycle(graph, lookup);
    for c in cycles {
        match c {
            DbError::Cycle(ErrorRef { inner, .. }) => {
                output.errors.push(SerializedError::Cycle(inner));
            }
        }
    }
    for (md_src, items) in items {
        if let Some(pb) = &md_src.md_doc_src.input_file {
            let pb = pb.clone();
            output.docs.entry(pb).or_insert(md_src.md_doc_src.clone());
        }
        for item in items {
            output.items.push(item.clone()); // todo: How to remove this clone...
        }
    }
    output
}

fn detect_cycle<'a>(
    graph: &'a HashMap<&'a String, Vec<&'a LineMarker<String>>>,
    lookup: &'a HashMap<&'a String, (&'a MdSrc<'a>, &'a Item)>,
) -> Vec<DbError<'a>> {
    let mut output: Vec<DbError> = vec![];
    for (parent_name, list_of_names) in graph {
        for child_name_marker in list_of_names {
            let child_name = &child_name_marker.item;
            if let Some(child_list) = graph.get(child_name) {
                let found = child_list.iter().find(|n| n.item == **parent_name);
                if let Some(_child_m) = found {
                    if let Some((src, item)) = lookup.get(parent_name) {
                        let cycle_err = CycleError::new(*parent_name, (*child_name_marker).clone());
                        let db_err = cycle_err.into_db_error(src, item);
                        output.push(db_err);
                    }
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::context::Context;

    use super::*;

    #[test]
    fn test_doc_from_src() {
        let ctx = Context::default();
        let f = ctx.read_docs_unwrapped(&vec![
            PathBuf::from("../fixtures/md/topics.md"),
            PathBuf::from("../fixtures/md/topics_2.md"),
            PathBuf::from("../fixtures/md/commands.md"),
        ]);
        let db = try_from_docs(&f, &OutputKind::Json);
        dbg!(db);
    }
}
