#[derive(Debug, serde::Deserialize)]
pub struct DependencyCheck {
    pub verify: String,
    pub autofix: Option<String>,
    pub url: String,
}
