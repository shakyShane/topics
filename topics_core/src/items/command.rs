#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Command {
    pub cwd: String,
    pub command: String,
    pub name: String,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            cwd: "./".to_string(),
            command: "echo 'hello world'".to_string(),
            name: "run unit tests command".to_string(),
        }
    }
}