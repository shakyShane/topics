use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FileExistsCheck {
    pub cwd: PathBuf,
    pub path: PathBuf,
    pub name: String,
}
