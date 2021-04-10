use crate::items::ItemWrap;

#[derive(Debug, Clone)]
pub struct Topic {
    pub name: String,
    pub steps: Vec<ItemWrap>,
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
