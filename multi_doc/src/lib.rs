use std::cmp::Ordering;
use typescript_definitions::TypeScriptify;
use yaml_rust::yaml;

#[derive(Debug, Clone, Default, serde::Serialize, TypeScriptify)]
pub struct MultiDoc {
    pub items: Vec<SingleDoc>,
}

#[derive(Debug, Clone, serde::Serialize, TypeScriptify)]
pub struct SingleDoc {
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
}

#[derive(Debug)]
enum DocKind {
    Yaml,
    Md,
    Unknown,
}

/// Create from a str
impl std::str::FromStr for MultiDoc {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, DocKind::Unknown)
    }
}

impl MultiDoc {
    pub fn from_yaml_str(input: &str) -> Result<Self, anyhow::Error> {
        Self::parse(input, DocKind::Yaml)
    }
    pub fn from_md_str(input: &str) -> Result<Self, anyhow::Error> {
        Self::parse(input, DocKind::Md)
    }
    pub fn from_any_str(input: &str) -> Result<Self, anyhow::Error> {
        Self::parse(input, DocKind::Unknown)
    }
    #[allow(clippy::unnecessary_wraps)]
    fn parse(str: &str, kind: DocKind) -> Result<Self, anyhow::Error> {
        let mut items: Vec<SingleDoc> = vec![];
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
                match end.cmp(&start) {
                    Ordering::Less => docs.push((start, line + 1)),
                    Ordering::Equal => {}
                    Ordering::Greater => docs.push((start, end)),
                }
            }
        }
        if start == 0 && end == 0 {
            docs.push((start, end_line))
        }

        for (start, end) in docs {
            let content = split[start..end].join("\n");
            match kind {
                DocKind::Yaml => {
                    let docs = yaml::YamlLoader::load_from_str(&content);
                    // we want to skip blank documents only,
                    // so we allow syntax errors (which will be handled later by serde)
                    // but we skip docs with no useful content
                    match docs {
                        Ok(vec) => {
                            if vec.get(0).is_some() {
                                items.push(SingleDoc {
                                    line_start: start,
                                    line_end: end,
                                    content,
                                });
                            }
                        }
                        Err(_e) => {
                            println!("scan error...");
                            items.push(SingleDoc {
                                line_start: start,
                                line_end: end,
                                content,
                            });
                        }
                    }
                }
                DocKind::Md | DocKind::Unknown => {
                    items.push(SingleDoc {
                        line_start: start,
                        line_end: end,
                        content,
                    });
                }
            }
        }

        Ok(Self { items })
    }
}

#[cfg(test)]
mod test {

    use super::*;

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
        let doc = MultiDoc::from_yaml_str(input)?;
        assert_eq!(doc.items.len(), 1);
        Ok(())
    }
    #[test]
    fn test_empty_doc() -> anyhow::Result<()> {
        let input = r#"

"#;
        let doc = MultiDoc::from_yaml_str(input)?;
        assert_eq!(doc.items.len(), 0);
        Ok(())
    }
    #[test]
    fn test_empty_doc_preserve_src() -> anyhow::Result<()> {
        let input = r#"

---

---
--- # a comment

---

name: "kittie"
---
"#;
        let doc = MultiDoc::from_yaml_str(input)?;
        insta::assert_debug_snapshot!(doc);
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
        let srcs = MultiDoc::from_yaml_str(input)?;
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
        let srcs = MultiDoc::from_yaml_str(input)?;
        insta::assert_debug_snapshot!(srcs);
        Ok(())
    }
    #[test]
    fn test_single_doc_md() -> anyhow::Result<()> {
        let input = r#"# Topic: Testing Rust code
"#;
        let srcs = MultiDoc::from_md_str(input)?;
        assert_eq!(srcs.items.len(), 1);
        Ok(())
    }
}
