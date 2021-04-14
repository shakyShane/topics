use crate::items::item::ItemWrap;

#[derive(Debug, Clone)]
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
