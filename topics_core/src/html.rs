use crate::context::Context;
use crate::doc::Doc;
use crate::doc_src::MdSrc;
use crate::html_template::HtmlTemplate;
use crate::items::{Item, LineMarker};
use crate::print::Print;
use crate::{DbError, ErrorRef, Output, SerializedError};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlOutput {
    pub pages: Vec<HtmlPage>,
    pub assets: Vec<Asset>,
}

impl Default for HtmlOutput {
    fn default() -> Self {
        Self {
            pages: Default::default(),
            assets: vec![
                Asset {
                    pb: "css/all.css".into(),
                    content: Some(CSS_FILE.into()),
                },
                Asset {
                    pb: "js/all.js".into(),
                    content: Some(JS_FILE.into()),
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct HtmlPage {
    pub pb: PathBuf,
    pub content: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug)]
pub struct Asset {
    pub pb: PathBuf,
    pub content: Option<String>,
}

const HTML_PAGE: &str = include_str!("../../web/index.html");
const CSS_FILE: &str = include_str!("../../web/css/all.css");
const JS_FILE: &str = include_str!("../../web/js/all.js");

impl HtmlTemplate for HtmlPage {
    fn template(&self, ctx: &Context) -> anyhow::Result<String> {
        Ok(HTML_PAGE
            .replace("{{title}}", "oops!")
            .replace("{{content}}", "<h1>MY page</h1>")
            .to_string())
    }
}

impl HtmlPage {
    pub fn new(pb: impl Into<PathBuf>) -> Self {
        Self {
            pb: pb.into(),
            content: None,
            title: None,
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn finish(self) -> Self {
        Self { ..self }
    }
}

pub fn output_html<'a>(
    graph: &'a HashMap<&'a String, Vec<&'a LineMarker<String>>>,
    lookup: &'a HashMap<&'a String, (&'a MdSrc<'a>, &'a Item)>,
    items: &'a Vec<(&'_ MdSrc, Vec<Item>)>,
) -> HtmlOutput {
    // let mut output = Output::default();
    // let cycles = detect_cycle(graph, lookup);
    // for c in cycles {
    //     match c {
    //         DbError::Cycle(ErrorRef { inner, .. }) => {
    //             output.errors.push(SerializedError::Cycle(inner));
    //         }
    //     }
    // }
    // for (md_src, items) in items {
    //     if let Some(pb) = &md_src.md_doc_src.input_file {
    //         let pb = pb.clone();
    //         output.docs.entry(pb).or_insert(md_src.md_doc_src.clone());
    //     }
    //     for item in items {
    //         output.items.push(item.clone()); // todo: How to remove this clone...
    //     }
    // }
    // output
    let p1 = HtmlPage::new("index.html")
        .title("Topic: hello!")
        .content("<!--Hello-->")
        .finish();
    let mut output = HtmlOutput::default();
    output.pages.push(p1);
    output
}
