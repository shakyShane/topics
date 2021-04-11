use crate::doc::DocResult;
use crate::doc_src::to_elements;
use crate::items::Item;
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode};
use comrak::{parse_document, Arena, ComrakOptions};
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

pub struct MdElements<'a> {
    arena: &'a Arena<AstNode<'a>>,
    pub root: &'a AstNode<'a>,
    pub items: Vec<Item>,
}

impl fmt::Debug for MdElements<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MdElements")
            .field("arena", &"Arena<AstNode<'a>>")
            .field("root", &"&'a AstNode<'a>")
            .field("items", &self.items)
            .finish()
    }
}

impl Deref for MdElements<'_> {
    type Target = Vec<Item>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'a> MdElements<'a> {
    pub fn new(string: &str, arena: &'a Arena<AstNode<'a>>) -> Self {
        let root = parse_document(arena, string, &ComrakOptions::default());
        Self {
            arena: &arena,
            root,
            items: vec![],
        }
    }
}

impl<'a> MdElements<'a> {
    pub fn select(&self, path: &[usize], len: usize) -> Vec<&'a Node<'_, RefCell<Ast>>> {
        if path.len() == 1 {
            if *path.get(0).unwrap() == 0 as usize {
                return self
                    .root
                    .children()
                    .take(len)
                    .collect::<Vec<&'_ Node<'_, RefCell<Ast>>>>();
            }
        }
        self.root
            .children()
            .take(1)
            .collect::<Vec<&'_ Node<'_, RefCell<Ast>>>>()
    }
}

#[test]
fn md_elements_lifetimes() {
    // let md = MdElements::new("# hi!", arena);
    let input = "# heading";
    let arena = Arena::new();
    let md_elements = MdElements::new(input, &arena);
    let md_elements_2 = MdElements::new(input, &arena);
}
