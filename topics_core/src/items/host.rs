use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, PartialEq, serde::Serialize, TypeScriptify)]
pub struct HostEntriesCheck {
    pub hosts: Vec<HostEntry>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, TypeScriptify)]
pub struct HostEntry {
    pub domain: String,
}
