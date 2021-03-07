#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DependencyCheck {
    pub verify: String,
    pub name: String,
    pub autofix: Option<String>,
    pub url: Option<String>,
}
