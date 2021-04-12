use crate::host::HostEntriesCheck;
use crate::items::Topic;
use crate::items::{Command, Instruction};
use crate::items::{DependencyCheck, TaskGroup};
use crate::items::{FileExistsCheck, LineMarker};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Item {
    Command(Command),
    FileExistsCheck(FileExistsCheck),
    DependencyCheck(DependencyCheck),
    Instruction(Instruction),
    HostEntriesCheck(HostEntriesCheck),
    Topic(Topic),
    TaskGroup(TaskGroup),
}

#[derive(Debug, Clone)]
pub enum ItemWrap {
    Named(String),
    Item(Item),
}

impl Item {
    pub fn set_name(&mut self, name: &str) {
        match self {
            Item::Command(cmd) => cmd.name = name.into(),
            Item::FileExistsCheck(fec) => fec.name = name.to_string(),
            Item::DependencyCheck(dc) => dc.name = name.to_string(),
            Item::Instruction(inst) => inst.name = name.into(),
            Item::HostEntriesCheck(hec) => hec.name = name.to_string(),
            Item::Topic(top) => top.name = name.into(),
            Item::TaskGroup(tg) => tg.name = name.to_string(),
        };
    }
    pub fn name(&self) -> String {
        match self {
            Item::Command(cmd) => cmd.name.to_string(),
            Item::FileExistsCheck(fec) => fec.name.clone(),
            Item::DependencyCheck(dc) => dc.name.clone(),
            Item::Instruction(inst) => inst.name.to_string(),
            Item::HostEntriesCheck(hec) => hec.name.clone(),
            Item::Topic(top) => top.name.to_string(),
            Item::TaskGroup(tg) => tg.name.clone(),
        }
    }
    pub fn kind_name(&self) -> String {
        match self {
            Item::Command(_) => "Command",
            Item::FileExistsCheck(_) => "File Exists Check",
            Item::DependencyCheck(_) => "Dependency Check",
            Item::Instruction(_) => "Instruction",
            Item::HostEntriesCheck(_) => "Host Entries Check",
            Item::Topic(_) => "Topic",
            Item::TaskGroup(_) => "Task Group",
        }
        .to_string()
    }
    pub fn set_line_start(&mut self, line_start: usize) {
        match self {
            Item::Instruction(inst) => inst.name.set_line_start(line_start),
            Item::Command(cmd) => cmd.name.set_line_start(line_start),
            Item::Topic(topic) => topic.name.set_line_start(line_start),
            _i => todo!("set line start {}", _i.name()),
        }
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FileExistsCheck" | "fec" => Ok(Item::FileExistsCheck(Default::default())),
            "Topic" | "topic" => Ok(Item::Topic(Default::default())),
            "TaskGroup" | "tg" | "task-group" => Ok(Item::TaskGroup(Default::default())),
            "Command" | "command" | "cmd" => Ok(Item::Command(Default::default())),
            "Instruction" | "inst" | "instruction" => Ok(Item::Instruction(Default::default())),
            "DependencyCheck" | "Dependency Check" | "dep" | "dep-check" => {
                Ok(Item::DependencyCheck(Default::default()))
            }
            _s => Err(anyhow::anyhow!("Not supported yet: {}", _s)),
        }
    }
}
