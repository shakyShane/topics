use crate::context::Context;
use crate::doc::Doc;
use crate::output::OutputDoc;
use crate::print::Print;
use crate::step::Step;
use anyhow::Result;
use bat::{Input, PrettyPrinter};
use std::fmt::Write;

#[derive(Debug)]
pub struct MdPrinter;

impl Print for MdPrinter {
    fn print(&self, d: &Doc, ctx: &Context) -> anyhow::Result<()> {
        println!("{:?}", self);
        let output = Self::print_md_doc(d, ctx.opts.index)?;
        PrettyPrinter::new()
            .header(true)
            // .grid(true)
            // .line_numbers(true)
            .inputs(vec![Input::from_bytes(output.body.as_bytes())
                .name("topics.md") // Dummy name provided to detect the syntax.
                .kind("File")
                .title(output.title)])
            .print()
            .unwrap();
        Ok(())
    }
}

impl MdPrinter {
    pub fn print_md_doc(doc: &Doc, index: Option<usize>) -> Result<OutputDoc> {
        let mut output = String::new();
        for (i, item) in doc.topics.iter().enumerate() {
            if let Some(index) = index {
                if i == index {
                    let _ = writeln!(output, "# Topic ({}) `{}`", i, item.name);
                    let _ = MdPrinter::print_steps(&mut output, &item.steps);
                }
            } else {
                let _ = writeln!(output, "# Topic ({}) `{}`", i, item.name);
                let _ = MdPrinter::print_steps(&mut output, &item.steps);
            }
        }
        Ok(OutputDoc::new(String::from("Oops"), output))
    }

    fn print_steps(str: &mut String, steps: &Vec<Step>) -> Result<()> {
        for step in steps {
            match step {
                Step::Command(cmd) => {
                    let _ = writeln!(str, "Command: **{}**", cmd.title)?;
                    let _ = writeln!(str, "- directory: `{}`", cmd.cwd)?;
                    let _ = writeln!(str, "```shell")?;
                    let _ = writeln!(str, "{}", cmd.command)?;
                    let _ = writeln!(str, "```\n")?;
                }
                Step::FileExistsCheck(fe) => {
                    let _ = writeln!(str, "FileExistsCheck")?;
                    let _ = writeln!(str, "- directory: `{}`", fe.cwd.display())?;
                    let _ = writeln!(str, "- file: `{}`\n", fe.path.display())?;
                }
                Step::DependencyCheck(dep) => {
                    let _ = writeln!(str, "DependencyCheck [{url}]({url})", url = dep.url)?;
                    let _ = writeln!(str, "```shell")?;
                    let _ = writeln!(str, "{}", dep.verify)?;
                    let _ = writeln!(str, "```\n")?;
                }
                Step::MultiSteps(multi) => MdPrinter::print_steps(str, &multi.steps)?,
                Step::Instruction(instr) => {
                    let _ = writeln!(str, "{}", instr.instruction)?;
                }
                Step::HostEntriesCheck(he) => {
                    let _ = writeln!(str, "HostEntriesCheck: {} domains", he.hosts.len())?;
                    for entry in &he.hosts {
                        let _ = writeln!(str, "- `127.0.0.1  {}`", entry.domain)?;
                    }
                    let _ = writeln!(str)?;
                }
            }
        }
        Ok(())
    }
}
