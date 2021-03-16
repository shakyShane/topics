use std::cmp::Ordering;
use yaml_rust::{yaml};

#[derive(Debug, Default)]
pub struct MultiYaml {
    pub items: Vec<YamlDoc>,
}

#[derive(Debug, Clone)]
pub struct YamlDoc {
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
}

/// Create from a str
impl std::str::FromStr for MultiYaml {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl MultiYaml {
    #[allow(clippy::unnecessary_wraps)]
    fn parse(str: &str) -> Result<Self, anyhow::Error> {
        let mut items: Vec<YamlDoc> = vec![];
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
            let docs = yaml::YamlLoader::load_from_str(&content);
            // we want to skip blank documents only,
            // so we allow syntax errors (which will be handled later by serde)
            // but we skip docs with no useful content
            match docs {
                Ok(vec) => {
                    if let Some(_) = vec.get(0) {
                        items.push(YamlDoc {
                            line_start: start,
                            line_end: end,
                            content,
                        });
                    }
                }
                Err(_e) => {
                    println!("scan error...");
                    items.push(YamlDoc {
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
        let doc = MultiYaml::from_str(input)?;
        assert_eq!(doc.items.len(), 1);
        Ok(())
    }
    #[test]
    fn test_empty_doc() -> anyhow::Result<()> {
        let input = r#"

"#;
        let doc = MultiYaml::from_str(input)?;
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
        let doc = MultiYaml::from_str(input)?;
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
        let srcs = MultiYaml::from_str(input)?;
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
        let srcs = MultiYaml::from_str(input)?;
        insta::assert_debug_snapshot!(srcs);
        Ok(())
    }
}
