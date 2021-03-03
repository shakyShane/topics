use crate::context::Context;
use crate::doc::Doc;
use crate::print::Print;

#[derive(Debug)]
pub struct PlainPrinter;

impl Print for PlainPrinter {
    fn print(&self, d: &Doc, _ctx: &Context) -> anyhow::Result<()> {
        let topic_len = d.topics.len();
        if let Some(src) = &d.source {
            println!(
                "\n{} Topic{} from `{}`",
                topic_len,
                if topic_len == 1 { "" } else { "s" },
                src.original.display()
            );
        } else {
            println!("\n{} Topics", topic_len);
        }
        for (index, topic) in d.topics.iter().enumerate() {
            println!("- {}) {}", index, topic.name);
        }
        Ok(())
    }
    fn print_all(&self, docs: &Vec<anyhow::Result<Doc>>, ctx: &Context) -> anyhow::Result<()> {
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
}
