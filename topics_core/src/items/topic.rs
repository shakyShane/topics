use crate::items::{ItemWrap, LineMarker};

#[derive(Debug, Clone)]
pub struct Topic {
    pub name: LineMarker<String>,
    pub steps: Vec<ItemWrap>,
    pub deps: Vec<ItemWrap>,
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            name: LineMarker::default(),
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
