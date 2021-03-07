use crate::host::HostEntriesCheck;
use crate::items::{DependencyCheck, TaskGroup};
use crate::items::FileExistsCheck;
use crate::items::Topic;
use crate::items::{Command, Instruction};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Item {
    Command(Command),
    FileExistsCheck(FileExistsCheck),
    DependencyCheck(DependencyCheck),
    Instruction(Instruction),
    HostEntriesCheck(HostEntriesCheck),
    Topic(Topic),
    TaskGroup(TaskGroup),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
            Item::TaskGroup(tg) => tg.name.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_item() -> Item {
        let input = r#"
        kind: Topic
        name: run tests
        steps:
            - kind: Instruction
              name: call
              instruction: Call your manager
        "#;
        serde_yaml::from_str(input).expect("test doesn't fail")
    }

    #[test]
    fn test_deserialize() -> anyhow::Result<()> {
        let _ = test_item();
        Ok(())
    }

    #[test]
    fn test_serialize() -> anyhow::Result<()> {
        // let t = test_item();
        let item = Item::Topic(Topic::default());
        let as_str = serde_yaml::to_string(&item)?;
        println!("|{}|", as_str);

        Ok(())
    }
}
