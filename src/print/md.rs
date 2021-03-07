use crate::context::Context;
use crate::doc::Doc;

// use crate::output::OutputDoc;
use crate::print::Print;

#[derive(Debug)]
pub struct MdPrinter;

impl Print for MdPrinter {
    fn print(&self, _d: &Doc, _ctx: &Context) -> anyhow::Result<()> {
        println!("{:?}", self);
        // let output = Self::print_md_doc(d, ctx.opts.index)?;
        // PrettyPrinter::new()
        //     .header(true)
        //     // .grid(true)
        //     // .line_numbers(true)
        //     .inputs(vec![Input::from_bytes(output.body.as_bytes())
        //         .name("topics.md") // Dummy name provided to detect the syntax.
        //         .kind("File")
        //         .title(output.title)])
        //     .print()
        //     .unwrap();
        Ok(())
    }

    fn print_welcome(&self, _docs: &Vec<Doc>, _ctx: &Context) -> anyhow::Result<()> {
        println!("later..");
        Ok(())
    }

    fn print_error(&self, _msg: &str, _ctx: &Context) {
        unimplemented!()
    }
}

impl MdPrinter {
    // pub fn print_md_doc(doc: &Doc, index: Option<usize>) -> Result<OutputDoc> {
    //     let output = String::new();
    //     for (i, _item) in doc.topics.iter().enumerate() {
    //         if let Some(index) = index {
    //             if i == index {
    //                 // let _ = writeln!(output, "# Topic ({}) `{}`", i, item.name);
    //                 // let _ = MdPrinter::print_steps(&mut output, &item.steps);
    //             }
    //         } else {
    //             // let _ = writeln!(output, "# Topic ({}) `{}`", i, item.name);
    //             // let _ = MdPrinter::print_steps(&mut output, &item.steps);
    //         }
    //     }
    //     Ok(OutputDoc::new(String::from("Oops"), output))
    // }
    //
    // fn print_steps(str: &mut String, steps: &Vec<Item>) -> Result<()> {
    //     for step in steps {
    //         match step {
    //             Item::Command(cmd) => {
    //                 let _ = writeln!(str, "Command: **{}**", cmd.name)?;
    //                 let _ = writeln!(str, "- directory: `{}`", cmd.cwd)?;
    //                 let _ = writeln!(str, "```shell")?;
    //                 let _ = writeln!(str, "{}", cmd.command)?;
    //                 let _ = writeln!(str, "```\n")?;
    //             }
    //             Item::FileExistsCheck(fe) => {
    //                 let _ = writeln!(str, "FileExistsCheck")?;
    //                 let _ = writeln!(str, "- directory: `{}`", fe.cwd.display())?;
    //                 let _ = writeln!(str, "- file: `{}`\n", fe.path.display())?;
    //             }
    //             Item::DependencyCheck(dep) => {
    //                 let _ = writeln!(str, "DependencyCheck [{url}]({url})", url = dep.url)?;
    //                 let _ = writeln!(str, "```shell")?;
    //                 let _ = writeln!(str, "{}", dep.verify)?;
    //                 let _ = writeln!(str, "```\n")?;
    //             }
    //             Item::Instruction(instr) => {
    //                 let _ = writeln!(str, "{}", instr.instruction)?;
    //             }
    //             Item::HostEntriesCheck(he) => {
    //                 let _ = writeln!(str, "HostEntriesCheck: {} domains", he.hosts.len())?;
    //                 for entry in &he.hosts {
    //                     let _ = writeln!(str, "- `127.0.0.1  {}`", entry.domain)?;
    //                 }
    //                 let _ = writeln!(str)?;
    //             }
    //             Item::Topic(_) => {}
    //         }
    //     }
    //     Ok(())
    // }
}
