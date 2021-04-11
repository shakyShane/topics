pub trait AstRangeImpl {
    fn ast_range(&self) -> AstRange;
}

///
/// An AST (tree-like) range selector
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// # use comrak::Arena;
/// # use topics_core::doc_src::MdElements;
/// let input = "# heading";
/// let arena = Arena::new();
/// let md_elements = MdElements::new(input, &arena);
///
/// // tuple of (vec![0], 1) is path + len
/// let html = md_elements.as_html((vec![0], 1));
/// assert_eq!(html, String::from("<h1>heading</h1>\n"));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct AstRange {
    pub ast_path: Vec<usize>,
    pub ast_len: usize,
}

impl AstRange {
    pub fn range(path: &[usize], len: usize) -> Self {
        Self {
            ast_path: path.to_vec(),
            ast_len: len,
        }
    }
}

impl AstRangeImpl for AstRange {
    fn ast_range(&self) -> AstRange {
        self.clone()
    }
}

impl AstRangeImpl for (Vec<usize>, usize) {
    fn ast_range(&self) -> AstRange {
        AstRange {
            ast_path: self.0.to_vec(),
            ast_len: self.1,
        }
    }
}

impl AstRangeImpl for (&'_ Vec<usize>, usize) {
    fn ast_range(&self) -> AstRange {
        AstRange {
            ast_path: self.0.to_vec(),
            ast_len: self.1,
        }
    }
}
