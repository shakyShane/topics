use crate::context::Context;

use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct DocSource {
    pub file_content: String,
    pub doc_src_items: Vec<DocSourceItem>,
}

#[derive(Debug)]
pub struct DocSourceItem {
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
}

impl DocSource {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> anyhow::Result<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocSrcError::FileRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let items = Self::parse(&file_str)?;
        let new_self = Self {
            file_content: file_str,
            doc_src_items: items,
        };
        Ok(new_self)
    }
    pub fn parse(str: &str) -> anyhow::Result<Vec<DocSourceItem>> {
        let mut output: Vec<DocSourceItem> = vec![];
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
            output.push(DocSourceItem {
                line_start: start,
                line_end: end,
                content,
            });
        }
        Ok(output)
    }
}

impl FromStr for DocSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = Self::parse(&s)?;
        Ok(Self {
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
}

#[derive(Debug, thiserror::Error)]
enum DocSrcError {
    #[error(
    "FileRead error: could not read file `{}`\nFull path: {}",
    pb.display(),
    abs.display()
    )]
    FileRead {
        pb: PathBuf,
        abs: PathBuf,
        original: std::io::Error,
    },
}

#[cfg(test)]
mod test {

    use crate::context::Context;

    use crate::doc_src::DocSource;
    use std::env::current_dir;

    #[test]
    fn test_fixture_file() -> anyhow::Result<()> {
        let ctx = Context::from_vec(&[]);
        let pb = current_dir()?.join("fixtures2/topics.yaml");
        let d = DocSource::from_path_buf(&pb, &ctx)?;
        insta::assert_debug_snapshot!(d);
        Ok(())
    }

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
        let doc = DocSource::parse(input)?;
        assert_eq!(doc.len(), 1);
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
        let srcs = DocSource::parse(input)?;
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
        let srcs = DocSource::parse(input)?;
        insta::assert_debug_snapshot!(srcs);
        Ok(())
    }
}
