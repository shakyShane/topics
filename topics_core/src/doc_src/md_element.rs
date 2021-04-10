use crate::doc::DocResult;

pub struct MdElements {
    elements: Vec<Element>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ListItem(Vec<Element>);
