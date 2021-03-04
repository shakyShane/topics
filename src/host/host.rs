#[derive(Debug, Clone, serde::Deserialize)]
pub struct HostEntriesCheck {
    pub hosts: Vec<HostEntry>,
    pub name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct HostEntry {
    pub domain: String,
}
