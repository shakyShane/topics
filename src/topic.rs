use crate::step::Step;

#[derive(Debug, serde::Deserialize)]
pub struct Topic {
    pub name: String,
    pub steps: Vec<Step>,
}
