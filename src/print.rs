use anyhow::Result;
use bat::{Input, PrettyPrinter};
use std::path::PathBuf;
use std::fmt::Write;
use crate::output::OutputDoc;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct Command {
    cwd: String,
    command: String,
    title: String,
}

#[derive(Debug, serde::Deserialize)]
struct FileExistsCheck {
    cwd: PathBuf,
    path: PathBuf,
}

#[derive(Debug, serde::Deserialize)]
struct DependencyCheck {
    verify: String,
    autofix: Option<String>,
    url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Doc {
    topics: Vec<Topic>,

    commands: Option<Vec<Step>>,
    steps: Option<Vec<Step>>,
    multi_steps: Option<Vec<Step>>,
}

#[derive(Debug, serde::Deserialize)]
struct Multi {
    steps: Vec<Step>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "kind")]
enum Step {
    Command(Command),
    FileExistsCheck(FileExistsCheck),
    DependencyCheck(DependencyCheck),
    MultiSteps(Multi),
    Instruction(Instruction),
}

#[derive(Debug, serde::Deserialize)]
struct Instruction {
    instruction: String
}

#[derive(Debug, serde::Deserialize)]
struct Topic {
    name: String,
    steps: Vec<Step>,
}

pub fn print_doc(doc: Doc, index: Option<usize>) -> Result<OutputDoc> {
    let mut output = String::new();
    for (i, item) in doc.topics.iter().enumerate() {
        if let Some(index) = index {
            if i == index {
                writeln!(output, "# Topic ({}) `{}`", i, item.name);
                let _ = print_steps(&mut output, &item.steps);
            }
        } else {
            writeln!(output, "# Topic ({}) `{}`", i, item.name);
            let _ = print_steps(&mut output, &item.steps);
        }
    }
    Ok(OutputDoc::new(String::from("Oops"), output))
}

fn print_steps(str: &mut String, steps: &Vec<Step>) -> Result<()> {
    for step in steps {
        match step {
            Step::Command(cmd) => {
                writeln!(str, "Command: **{}**", cmd.title)?;
                writeln!(str, "- directory: `{}`", cmd.cwd)?;
                writeln!(str, "```shell")?;
                writeln!(str, "{}", cmd.command)?;
                writeln!(str, "```\n")?;
            }
            Step::FileExistsCheck(fe) => {
                writeln!(str, "FileExistsCheck")?;
                writeln!(str, "- directory: `{}`", fe.cwd.display())?;
                writeln!(str, "- file: `{}`\n", fe.path.display())?;
            }
            Step::DependencyCheck(dep) => {
                writeln!(str, "DependencyCheck [{url}]({url})", url = dep.url)?;
                writeln!(str, "```shell")?;
                writeln!(str, "{}", dep.verify)?;
                writeln!(str, "```\n")?;
            }
            Step::MultiSteps(multi) => print_steps(str, &multi.steps)?,
            Step::Instruction(instr) => {
                writeln!(str, "{}", instr.instruction)?;
            }
        }
    }
    Ok(())
}
