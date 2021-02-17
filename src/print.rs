use anyhow::Result;

use std::path::PathBuf;

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

pub fn print_doc(doc: Doc, index: Option<usize>) -> Result<()> {
    doc.topics.iter().enumerate().for_each(|(i, item)| {
        if let Some(index) = index {
            if i == index {
                println!("-----");
                println!("Topic ({}) `{}`", i, item.name);
                println!("-----");

                print_steps(&item.steps);
            }
        } else {
            println!("-----");
            println!("Topic: ({}) `{}`", i, item.name);
            println!("-----");
            print_steps(&item.steps);
        }
    });
    Ok(())
}

fn print_steps(steps: &Vec<Step>) {
    steps.iter().for_each(|step| match step {
        Step::Command(cmd) => {
            println!("[cmd] {}", cmd.title);
            println!("  - [dir] {}", cmd.cwd);
            println!("  - [run] \n\n\t{}\n", cmd.command);
        }
        Step::FileExistsCheck(fe) => {
            println!("file check: {}", fe.path.display())
        }
        Step::DependencyCheck(dep) => {
            println!("[dep] {}", dep.verify)
        }
        Step::MultiSteps(multi) => print_steps(&multi.steps),
        Step::Instruction(instr) => {
            println!("{}", instr.instruction)
        }
    });
}
