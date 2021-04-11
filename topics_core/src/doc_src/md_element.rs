use crate::doc::DocResult;
use crate::doc_src::{process_node, to_items};
use crate::items::Item;
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

///
/// The purpose of this is to maintain FULL AST information
/// about the given markdown. 'items', such as a Command will
/// be parsed initially to discover their semantic meaning (eg: a Command
/// requires at LEAST a name + code fence).
///
/// As part of this 'initial parse', it will maintain a lookup mechanism to
/// allow referring back to the original AST -> this is useful in cases like later
/// steps requiring any more fine-grained information (such a line numbers)
///
/// This also allows sub-sections of markdown to be converted into HTML as needed
/// (or IF needed)
///
/// The lifetimes/references are here to ensure we only ever parse/store an AST once,
/// but multiple items can refer back to it as needed via path + length offsets.
///
/// Eg: an 'Instruction' is technically an entire markdown file that contains a heading title
/// followed by n lines of markdown.
///
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
    pub fn as_items<'b>(&self) -> Result<Vec<Item>, anyhow::Error> {
        let mut path = vec![0];
        let items = process_node(&self.root, &mut path);
        Ok(items)
    }
    pub fn select_ast(&self, path: &'a [usize], len: usize) -> Vec<&'a Node<'a, RefCell<Ast>>> {
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
    pub fn as_html(&self, path: &'a [usize], len: usize) -> String {
        let nodes = self.select_ast(&path, len);
        let mut output = vec![];
        for node in nodes {
            let mut options = ComrakOptions::default();
            options.render.unsafe_ = true;
            let res = format_html(node, &options, &mut output);
            if let Err(e) = res {
                eprintln!("{:?}", e)
            }
        }
        std::str::from_utf8(&*output)
            .expect("Valid UTF8 expected")
            .to_string()
    }
}

impl<'a> TryFrom<&'a MdElements<'a>> for Vec<Item> {
    type Error = anyhow::Error;

    fn try_from(md: &'a MdElements<'a>) -> Result<Self, Self::Error> {
        md.as_items()
    }
}

#[test]
fn md_elements() {
    // let md = MdElements::new("# hi!", arena);
    let input = "# heading";
    let arena = Arena::new();
    let md_elements = MdElements::new(input, &arena);
    let html = md_elements.as_html(&vec![0], 1);
    assert_eq!(html, String::from("<h1>heading</h1>\n"));
}
