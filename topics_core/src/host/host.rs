#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HostEntriesCheck {
    pub hosts: Vec<HostEntry>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HostEntry {
    pub domain: String,
}
