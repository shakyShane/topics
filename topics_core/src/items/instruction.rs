#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Instruction {
    pub instruction: String,
    pub name: String,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            instruction: "".to_string(),
            name: "".to_string(),
        }
    }
}
