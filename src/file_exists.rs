use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct FileExistsCheck {
    pub cwd: PathBuf,
    pub path: PathBuf,
}
