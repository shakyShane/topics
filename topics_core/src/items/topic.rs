use crate::items::ItemWrap;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Topic {
    pub name: String,
    pub steps: Vec<ItemWrap>,
    #[serde(default)]
    pub deps: Vec<ItemWrap>,
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            name: "[sample] Run unit tests".to_string(),
            deps: vec![
                // ItemWrap::Named("install node".to_string()),
                // ItemWrap::Named("install yarn".to_string()),
            ],
            steps: vec![
                // ItemWrap::Named("run unit tests command".into())
            ],
        }
    }
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
