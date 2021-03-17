use crate::context::Context;
use crate::doc::{Doc, DocResult};
use crate::doc_err::{DocError, Location, LocationError};
use crate::doc_src::DocSrcImpl;
use multi_doc::{MultiDoc, SingleDoc};
use std::path::PathBuf;
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new("at line (\\d+)").unwrap();
}

#[derive(Debug, Clone, Default)]
pub struct YamlDocSource {
    pub input_file: Option<PathBuf>,
    pub file_content: String,
    pub doc_src_items: MultiDoc,
}

impl DocSrcImpl for YamlDocSource {
    fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocError::PathRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let items = MultiDoc::from_yaml_str(&file_str)?;
        let new_self = Self {
            input_file: Some(pb.clone()),
            file_content: file_str,
            doc_src_items: items,
        };
        Ok(new_self)
    }
}

impl FromStr for YamlDocSource {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = MultiDoc::from_yaml_str(&s)?;
        Ok(Self {
            input_file: None,
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
}

pub fn from_serde_yaml_error(
    doc: &Doc,
    single_doc: &SingleDoc,
    serde_error: &serde_yaml::Error,
) -> DocError {
    let mut err = LocationError {
        input_file_src: doc.source.content().to_string(),
        location: Some(Location::Region {
            line_start: single_doc.line_start + 1,
            line_end: single_doc.line_end,
        }),
        input_file: doc.source.file(),
        description: serde_error.to_string(),
    };
    if let Some(location) = serde_error.location() {
        let real_line = location.line() + single_doc.line_start;
        err.location = Some(Location::LineAndColRegion {
            line: real_line,
            column: location.column(),
            line_start: single_doc.line_start + 1,
            line_end: single_doc.line_end,
        });
        err.description = RE
            .replace_all(
                err.description.as_str(),
                format!("at line {}", real_line).as_str(),
            )
            .to_string()
    }
    DocError::SerdeLocationErr(err)
}

#[cfg(test)]
mod test {

    use crate::context::Context;
    use crate::doc::Doc;
    use crate::doc_src::{DocSource, DocSrcImpl, YamlDocSource};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_fixture_file() -> anyhow::Result<()> {
        let ctx = Context::from_vec(&[]);
        let pb = PathBuf::from("../fixtures2/topics.yaml");
        let d = YamlDocSource::from_path_buf(&pb, &ctx)?;
        insta::assert_debug_snapshot!(d);
        Ok(())
    }

    #[test]
    fn test_errors_single() -> anyhow::Result<()> {
        let pb = PathBuf::from("/input-yaml.yml");
        let input = r#"
kind: Topic
name: Run screen shot tests
deps
"#;
        let srcs = YamlDocSource::from_str(input)?;
        let doc = Doc::from_doc_src(&pb, DocSource::Yaml(srcs), &Default::default());
        insta::assert_debug_snapshot!(doc);
        Ok(())
    }

    #[test]
    fn test_errors_multi() -> anyhow::Result<()> {
        let pb = PathBuf::from("/input-yaml.yml");
        let input = r#"---

kind: DependencyCheck
name: global-node
verify: node -v
url: https://www.nodejs.org

---

kind: DependencyCheck
name: global-yarn
verify: yarn -v
url: https://yarn.sh/legacy

---

kind: Topic
name: Run screen shot tests
deps:
  - global-node
  - global-yarn
steps
"#;
        let srcs = YamlDocSource::from_str(input)?;
        let doc = Doc::from_doc_src(&pb, DocSource::Yaml(srcs), &Default::default());
        insta::assert_debug_snapshot!(doc?.errors);
        Ok(())
    }
}
