use crate::items::item::ItemWrap;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct TaskGroup {
    pub name: String,
    pub steps: Vec<ItemWrap>
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
