use crate::doc::DocResult;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MdElements {
    pub elements: Vec<Element>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Element {
    Heading {
        level: usize,
        content: String,
    },
    List {
        items: Vec<ListItem>,
    },
    Paragraph {
        content: String,
    },
    Text {
        content: String,
    },
    CodeBlock {
        params: Option<String>,
        content: String,
    },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListItem(pub(crate) Vec<Element>);

impl Element {
    pub fn h1(content: impl Into<String>) -> Self {
        Self::Heading {
            level: 1,
            content: content.into(),
        }
    }
    pub fn code_block(content: impl Into<String>, params: Option<impl Into<String>>) -> Self {
        Self::CodeBlock {
            content: content.into(),
            params: params.map(|p| p.into()),
        }
    }
    pub fn code_block_without_params(content: impl Into<String>) -> Self {
        Self::CodeBlock {
            content: content.into(),
            params: None,
        }
    }
}
