use crate::items::item::ItemWrap;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, serde::Serialize, TypeScriptify)]
pub struct TaskGroup {
    pub name: String,
    pub steps: Vec<ItemWrap>,
}

impl Default for TaskGroup {
    fn default() -> Self {
        Self {
            name: "Machine setup".to_string(),
            steps: vec![],
        }
    }
}
