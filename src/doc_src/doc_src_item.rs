use crate::doc::{DocError, DocResult};

#[derive(Debug, Default)]
pub struct DocSourceItems {
    pub items: Vec<DocSourceItem>,
}

#[derive(Debug)]
pub struct DocSourceItem {
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
}

/// Create from a str
impl std::str::FromStr for DocSourceItems {
    type Err = DocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl DocSourceItems {
    fn parse(str: &str) -> DocResult<Self> {
        let mut items: Vec<DocSourceItem> = vec![];
        let split = str.lines().collect::<Vec<&str>>();
        let mut peek = split.iter().enumerate().peekable();
        let mut start = 0;
        let mut end = 0;
        let mut end_line = 0;
        let mut docs: Vec<(usize, usize)> = vec![];
        while let Some((line, content)) = peek.next() {
            if content.starts_with("---") {
                let finish = if line > 0 { line - 1 } else { line };
                if start < finish {
                    docs.push((start, line));
                    end = line;
                }
                start = line + 1;
            } else if peek.peek().is_none() {
                end_line = line + 1;
                if end < start {
                    docs.push((start, line + 1))
                } else {
                    if end > start {
                        docs.push((start, end))
                    }
                }
            }
        }
        if start == 0 && end == 0 {
            docs.push((start, end_line))
        }

        for (start, end) in docs {
            let content = split[start..end].join("\n");
            items.push(DocSourceItem {
                line_start: start,
                line_end: end,
                content,
            });
        }

        Ok(Self { items })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_single_doc() -> anyhow::Result<()> {
        let input = r#"
kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:
  - github-checkin
"#;
        let doc = DocSourceItems::from_str(input)?;
        assert_eq!(doc.items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_single_doc_with_prefix_padding() -> anyhow::Result<()> {
        let input = r#"
---
---
kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:
  - github-checkin
"#;
        let srcs = DocSourceItems::from_str(input)?;
        insta::assert_debug_snapshot!(srcs);
        Ok(())
    }

    #[test]
    fn test_single_doc_with_suffix_padding() -> anyhow::Result<()> {
        let input = r#"
kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps:
  - github-checkin
  - |
    some other long string
---
---
"#;
        let srcs = DocSourceItems::from_str(input)?;
        insta::assert_debug_snapshot!(srcs);
        Ok(())
    }
}
