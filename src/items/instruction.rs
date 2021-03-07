#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Instruction {
    pub instruction: String,
    pub name: String,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            instruction: "Log into Github & create your PR in github".to_string(),
            name: "create a new PR".to_string(),
        }
    }
}
