use crate::context::Context;
use crate::doc::{Doc, DocResult};
use crate::items::topic::Topic;
use crate::print::{md, plain};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub trait Print: Debug {
    fn print(&self, _doc: &Doc, _ctx: &Context) -> anyhow::Result<()>;
    fn print_welcome(&self, _docs: &Vec<Doc>, _ctx: &Context) -> anyhow::Result<()>;
    fn print_heading(&self, kind: &str, message: &str) {
        println!("[default impl heading]");
        println!("{} {}", kind, message);
    }
    fn print_topic(&self, topic: &Topic, _doc: &Doc, _ctx: &Context) -> anyhow::Result<()> {
        println!("[default impl topic]");
        println!("{:?}", topic);
        Ok(())
    }
    fn print_all(&self, docs: &Vec<DocResult<Doc>>, _ctx: &Context) -> anyhow::Result<()> {
        println!("[default impl] printing {} doc(s)", docs.len());
        Ok(())
    }
    fn print_errors(&self, docs: &Vec<DocResult<Doc>>, _ctx: &Context) -> anyhow::Result<()> {
        println!("[default impl print::print_errors]");
        for doc in docs {
            match doc {
                Err(e) => {
                    eprintln!("{}", e);
                }
                Ok(doc) => {
                    for err in &doc.errors {
                        eprintln!("{}", err);
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrintKind {
    Plain,
    Markdown,
    Json,
}

impl Display for PrintKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for PrintKind {
    fn default() -> Self {
        Self::Plain
    }
}

impl Print for PrintKind {
    fn print(&self, d: &Doc, ctx: &Context) -> anyhow::Result<()> {
        match self {
            PrintKind::Markdown => (md::MdPrinter).print(d, ctx),
            PrintKind::Plain => (plain::PlainPrinter).print(d, ctx),
            PrintKind::Json => {
                todo!("implement json")
            }
        }
    }

    fn print_welcome(&self, docs: &Vec<Doc>, ctx: &Context) -> anyhow::Result<()> {
        match self {
            PrintKind::Markdown => (md::MdPrinter).print_welcome(docs, ctx),
            PrintKind::Plain => (plain::PlainPrinter).print_welcome(docs, ctx),
            PrintKind::Json => {
                todo!("implement json")
            }
        }
    }

    fn print_heading(&self, kind: &str, message: &str) {
        match self {
            PrintKind::Plain => (plain::PlainPrinter).print_heading(kind, message),
            _ => todo!("implement others for print_topic"),
        }
    }

    fn print_topic(&self, topic: &Topic, doc: &Doc, ctx: &Context) -> anyhow::Result<()> {
        match self {
            PrintKind::Plain => (plain::PlainPrinter).print_topic(topic, doc, ctx),
            _ => todo!("implement others for print_topic"),
        }
    }

    fn print_all(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        match self {
            PrintKind::Markdown => (md::MdPrinter).print_all(docs, ctx),
            PrintKind::Plain => (plain::PlainPrinter).print_all(docs, ctx),
            PrintKind::Json => {
                todo!("implement json")
            }
        }
    }
    fn print_errors(&self, docs: &Vec<DocResult<Doc>>, ctx: &Context) -> anyhow::Result<()> {
        match self {
            PrintKind::Markdown => (md::MdPrinter).print_errors(docs, ctx),
            PrintKind::Plain => (plain::PlainPrinter).print_errors(docs, ctx),
            PrintKind::Json => {
                todo!("implement json")
            }
        }
    }
}

impl FromStr for PrintKind {
    type Err = PrintKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" | "Markdown" => Ok(PrintKind::Markdown),
            "json" | "Json" => Ok(PrintKind::Json),
            "plain" | "Plain" => Ok(PrintKind::Plain),
            _a => {
                return Err(PrintKindError::Unknown);
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PrintKindError {
    #[error("print kind not recognised")]
    Unknown,
}
