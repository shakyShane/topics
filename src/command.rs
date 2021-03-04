#[derive(Debug, Clone, serde::Deserialize)]
pub struct Command {
    pub cwd: String,
    pub command: String,
    pub name: String,
}
