use crate::context::Context;
use crate::db::Db;
use crate::doc::{Doc, DocResult};
use crate::items::topic::Topic;
use crate::print::{md, plain};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub trait Print: Debug {
    fn print_welcome(&self, _docs: &[Doc], _ctx: &Context) -> anyhow::Result<()>;
    fn print_error(&self, msg: &str, _ctx: &Context) -> anyhow::Result<()>;
    fn print_heading(&self, kind: &str, message: &str) {
        println!("[default impl heading]");
        println!("{} {}", kind, message);
    }
    fn print_topic(
        &self,
        topic: &Topic,
        _b: &Db,
        _doc: &Doc,
        _ctx: &Context,
    ) -> anyhow::Result<()> {
        println!("[default impl topic]");
        println!("{:?}", topic);
        Ok(())
    }
    fn print_all(&self, docs: &[Doc], _b: &Db, _ctx: &Context) -> anyhow::Result<()> {
        println!("[default impl] printing {} doc(s)", docs.len());
        Ok(())
    }
    fn print_errors(&self, docs: &[DocResult<Doc>], _ctx: &Context) -> anyhow::Result<()> {
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
pub enum OutputKind {
    Plain,
    Markdown,
    Json,
}

impl Display for OutputKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for OutputKind {
    fn default() -> Self {
        Self::Plain
    }
}

impl Print for OutputKind {
    fn print_welcome(&self, docs: &[Doc], ctx: &Context) -> anyhow::Result<()> {
        match self {
            OutputKind::Markdown => (md::MdPrinter).print_welcome(docs, ctx),
            OutputKind::Plain => (plain::PlainPrinter).print_welcome(docs, ctx),
            OutputKind::Json => {
                todo!("implement json")
            }
        }
    }

    fn print_error(&self, msg: &str, ctx: &Context) -> anyhow::Result<()> {
        match self {
            OutputKind::Plain => (plain::PlainPrinter).print_error(msg, ctx),
            _ => todo!("implement others for print_topic"),
        }
    }

    fn print_heading(&self, kind: &str, message: &str) {
        match self {
            OutputKind::Plain => (plain::PlainPrinter).print_heading(kind, message),
            _ => todo!("implement others for print_topic"),
        }
    }

    fn print_topic(&self, topic: &Topic, db: &Db, doc: &Doc, ctx: &Context) -> anyhow::Result<()> {
        match self {
            OutputKind::Plain => (plain::PlainPrinter).print_topic(topic, db, doc, ctx),
            _ => todo!("implement others for print_topic"),
        }
    }

    fn print_all(&self, docs: &[Doc], db: &Db, ctx: &Context) -> anyhow::Result<()> {
        match self {
            OutputKind::Markdown => (md::MdPrinter).print_all(docs, &db, ctx),
            OutputKind::Plain => (plain::PlainPrinter).print_all(docs, &db, ctx),
            OutputKind::Json => {
                todo!("implement json")
            }
        }
    }
    fn print_errors(&self, docs: &[DocResult<Doc>], ctx: &Context) -> anyhow::Result<()> {
        match self {
            OutputKind::Markdown => (md::MdPrinter).print_errors(docs, ctx),
            OutputKind::Plain => (plain::PlainPrinter).print_errors(docs, ctx),
            OutputKind::Json => {
                todo!("implement json")
            }
        }
    }
}

impl FromStr for OutputKind {
    type Err = PrintKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" | "Markdown" => Ok(OutputKind::Markdown),
            "json" | "Json" => Ok(OutputKind::Json),
            "plain" | "Plain" => Ok(OutputKind::Plain),
            _a => Err(PrintKindError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PrintKindError {
    #[error("print kind not recognised")]
    Unknown,
}
