use std::collections::HashMap;
use std::path::PathBuf;

use crate::db_error::SerializedError;
use crate::doc_src::{MdDocSource, MdSrc};
use crate::html::HtmlOutput;
use crate::items::{Item, LineMarker};
use crate::{DbError, ErrorRef};
use typescript_definitions::TypeScriptify;

pub enum Outputs {
    Plain(Output),
    Json(Output),
    Html(HtmlOutput),
}

#[derive(Debug, Default, TypeScriptify, serde::Serialize)]
pub struct Output {
    pub docs: HashMap<PathBuf, MdDocSource>,
    pub items: Vec<Item>,
    pub errors: Vec<SerializedError>,
}

pub fn output<'a>(
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
