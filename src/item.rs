use crate::command::Command;
use crate::dependency::DependencyCheck;
use crate::file_exists::FileExistsCheck;
use crate::host::HostEntriesCheck;
use crate::instruction::Instruction;
use crate::topic::Topic;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Item {
    Command(Command),
    FileExistsCheck(FileExistsCheck),
    DependencyCheck(DependencyCheck),
    Instruction(Instruction),
    HostEntriesCheck(HostEntriesCheck),
    Topic(Topic),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum ItemWrap {
    Named(String),
    Item(Item),
}

impl Item {
    pub fn name(&self) -> String {
        match self {
            Item::Command(cmd) => cmd.name.clone(),
            Item::FileExistsCheck(fec) => fec.name.clone(),
            Item::DependencyCheck(dc) => dc.name.clone(),
            Item::Instruction(inst) => inst.name.clone(),
            Item::HostEntriesCheck(hec) => hec.name.clone(),
            Item::Topic(top) => top.name.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() -> anyhow::Result<()> {
        let input = r#"
        kind: Topic
        name: run tests
        steps:
            - kind: Instruction
              name: call
              instruction: Call your manager
        "#;
        let t: Topic = serde_yaml::from_str(input)?;
        dbg!(t);
        Ok(())
    }
}
