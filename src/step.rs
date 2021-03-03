use crate::command::Command;
use crate::dependency::DependencyCheck;
use crate::file_exists::FileExistsCheck;
use crate::host::HostEntriesCheck;
use crate::instruction::Instruction;
use crate::multi_step::MultiStep;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Step {
    Command(Command),
    FileExistsCheck(FileExistsCheck),
    DependencyCheck(DependencyCheck),
    MultiSteps(MultiStep),
    Instruction(Instruction),
    HostEntriesCheck(HostEntriesCheck),
}
