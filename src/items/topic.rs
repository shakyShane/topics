use crate::items::item::ItemWrap;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Topic {
    pub name: String,
    pub steps: Vec<ItemWrap>,
    #[serde(default)]
    pub deps: Vec<ItemWrap>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let t = Topic::default();
        let yaml = serde_yaml::to_string(&t)?;
        println!("|{}|", yaml);
        Ok(())
    }
}
