#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Instruction {
    pub instruction: String,
    pub name: String,
}
