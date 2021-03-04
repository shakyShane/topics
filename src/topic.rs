use crate::item::ItemWrap;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Topic {
    pub name: String,
    pub steps: Vec<ItemWrap>,
    #[serde(default)]
    pub deps: Vec<ItemWrap>,
}
