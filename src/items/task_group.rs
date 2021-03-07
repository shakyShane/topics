use crate::items::item::ItemWrap;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TaskGroup {
    pub name: String,
    pub steps: Vec<ItemWrap>,
}

impl Default for TaskGroup {
    fn default() -> Self {
        Self {
            name: "Machine setup".to_string(),
            steps: vec![
                ItemWrap::Named("Kubernetes setup".to_string()),
                ItemWrap::Named("Frontend setup".to_string()),
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let t = TaskGroup::default();
        let yaml = serde_yaml::to_string(&t)?;
        println!("|{}|", yaml);
        Ok(())
    }
}
