use crate::step::Step;

#[derive(Debug, serde::Deserialize)]
pub struct MultiStep {
    pub steps: Vec<Step>,
}
