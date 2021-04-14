use crate::items::{ItemWrap, LineMarker};

#[derive(Debug, Clone)]
pub struct Topic {
    pub name: LineMarker<String>,
    pub steps: Vec<ItemWrap>,
    pub deps: Vec<ItemWrap>,
}

impl Topic {
    pub fn add_step_named_ref(&mut self, string: impl Into<String>, line_start: u32) {
        self.steps
            .push(ItemWrap::named_ref(string.into(), line_start));
    }
    pub fn add_dep_named_ref(&mut self, string: impl Into<String>, line_start: u32) {
        self.deps
            .push(ItemWrap::named_ref(string.into(), line_start));
    }
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
