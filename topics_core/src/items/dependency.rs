#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DependencyCheck {
    pub verify: String,
    pub name: String,
    pub autofix: Option<String>,
    pub url: Option<String>,
}

impl DependencyCheck {
    pub fn minimal(name: &str, verify: &str) -> Self {
        Self {
            verify: name.to_string(),
            name: verify.to_string(),
            autofix: None,
            url: None,
        }
    }
}

impl Default for DependencyCheck {
    fn default() -> Self {
        Self {
            verify: "node -v".to_string(),
            name: "install node".to_string(),
            autofix: None,
            url: Some("https://nodejs.org".to_string()),
        }
    }
}
