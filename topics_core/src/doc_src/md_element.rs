use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode};
use comrak::{format_html, parse_document, Arena, ComrakOptions};

use crate::doc::DocResult;
use crate::doc_src::ast_range::{AstRange, AstRangeImpl};
use crate::doc_src::{process_node, to_items, DocSource, MdDocSource};
use crate::items::Item;
use std::fmt::{Debug, Formatter};
use multi_doc::SingleDoc;

#[derive(Default)]
pub struct MdSrc<'a> {
    arena: Arena<AstNode<'a>>,

    pub md_doc_src: &'a MdDocSource,
    pub item_doc: &'a SingleDoc,
    pub md_elements: RefCell<Option<MdElements<'a>>>,
}

impl Debug for MdSrc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MdSrc")
            .field("arena", &"Arena<AstNode>")
            .field("md_doc_src", &self.md_doc_src)
            .field("md_elements", &self.md_elements)
            .finish()
    }
}

impl<'a> MdSrc<'a> {
    pub fn new(doc_src: &'a MdDocSource, single_doc: &'a SingleDoc) -> Self {
        let a = Arena::new();
        Self {
            arena: a,
            md_doc_src: doc_src,
            item_doc: single_doc,
            md_elements: RefCell::new(None),
        }
    }
    pub fn parse(&'a self) {
        *self.md_elements.borrow_mut() = Some(MdElements::new(self.md_doc_src., &self.arena));
    }
    pub fn items(&'a self) -> Vec<Item> {
        self.md_elements
            .borrow()
            .as_ref()
            .expect("must parse before getting here")
            .as_items()
    }
}

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
    items: Vec<Item>,
    pub root: &'a AstNode<'a>,
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
    pub fn as_items<'b>(&self) -> Vec<Item> {
        let mut path = vec![0];
        let items = process_node(&self.root, &mut path);
        items
    }
    pub fn select_ast(&self, range: impl AstRangeImpl) -> Vec<&'a Node<'a, RefCell<Ast>>> {
        let AstRange { ast_len, ast_path } = range.ast_range();
        if ast_path.len() == 1 {
            if *ast_path.get(0).unwrap() == 0 as usize {
                return self
                    .root
                    .children()
                    .take(ast_len)
                    .collect::<Vec<&'_ Node<'_, RefCell<Ast>>>>();
            }
        }
        if ast_path.len() == 2 {
            let second = ast_path.get(1).unwrap();
            return self
                .root
                .children()
                .skip((*second))
                .take(ast_len)
                .collect::<Vec<&'_ Node<'_, RefCell<Ast>>>>();
        }
        self.root
            .children()
            .take(1)
            .collect::<Vec<&'_ Node<'_, RefCell<Ast>>>>()
    }
    ///
    ///
    /// Convert an ast range into HTML
    ///
    /// ```rust
    /// use comrak::Arena;
    /// use topics_core::doc_src::MdElements;
    ///
    /// let input = "# heading";
    /// let arena = Arena::new();
    /// let md_elements = MdElements::new(input, &arena);
    /// let html = md_elements.as_html((vec![0], 1));
    ///
    /// assert_eq!(html, String::from("<h1>heading</h1>\n"));
    /// ```
    pub fn as_html(&self, range: impl AstRangeImpl) -> String {
        let nodes = self.select_ast(range);
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
        Ok(md.as_items())
    }
}
