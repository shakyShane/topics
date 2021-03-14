use crate::context::Context;

use crate::doc::{DocError, DocResult};
use multi_yaml::MultiYaml;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct DocSource {
    pub input_file: Option<PathBuf>,
    pub file_content: String,
    pub doc_src_items: MultiYaml,
}

impl DocSource {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocError::PathRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let items = MultiYaml::from_str(&file_str)?;
        let new_self = Self {
            input_file: Some(pb.clone()),
            file_content: file_str,
            doc_src_items: items,
        };
        Ok(new_self)
    }
}

impl FromStr for DocSource {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = MultiYaml::from_str(&s)?;
        Ok(Self {
            input_file: None,
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
}

#[cfg(test)]
mod test {

    use crate::context::Context;

    use crate::doc_src::DocSource;

    use std::path::PathBuf;

    #[test]
    fn test_fixture_file() -> anyhow::Result<()> {
        let ctx = Context::from_vec(&[]);
        let pb = PathBuf::from("../fixtures2/topics.yaml");
        let d = DocSource::from_path_buf(&pb, &ctx)?;
        insta::assert_debug_snapshot!(d);
        Ok(())
    }
}
