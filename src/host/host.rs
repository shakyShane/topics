#[derive(Debug, serde::Deserialize)]
pub struct HostEntriesCheck {
    pub hosts: Vec<HostEntry>,
}

#[derive(Debug, serde::Deserialize)]
pub struct HostEntry {
    pub domain: String,
}
