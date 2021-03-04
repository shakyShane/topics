#[derive(Debug, Clone, serde::Deserialize)]
pub struct DependencyCheck {
    pub verify: String,
    pub name: String,
    pub autofix: Option<String>,
    pub url: String,
}
