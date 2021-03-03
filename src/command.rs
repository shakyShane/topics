#[derive(Debug, serde::Deserialize)]
pub struct Command {
    pub cwd: String,
    pub command: String,
    pub title: String,
}
