#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Command {
    pub cwd: String,
    pub command: String,
    pub name: String,
}
