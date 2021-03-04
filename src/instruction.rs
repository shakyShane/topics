#[derive(Debug, Clone, serde::Deserialize)]
pub struct Instruction {
    pub instruction: String,
    pub name: String,
}
