use std::path::PathBuf;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, TypeScriptify)]
pub struct FileExistsCheck {
    pub cwd: PathBuf,
    pub path: PathBuf,
    pub name: String,
}

impl Default for FileExistsCheck {
    fn default() -> Self {
        Self {
            cwd: PathBuf::from("./"),
            path: PathBuf::from("Cargo.toml"),
            name: "Cargo.toml exists".to_string(),
        }
    }
}
