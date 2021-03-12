use crate::context::Context;

use crate::doc::{DocError, DocResult};
use crate::doc_src::DocSourceItems;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct DocSource {
    pub file_content: String,
    pub doc_src_items: DocSourceItems,
}

impl DocSource {
    pub fn from_path_buf(pb: &PathBuf, ctx: &Context) -> DocResult<Self> {
        let abs = ctx.join_path(pb);
        let file_str = std::fs::read_to_string(&abs).map_err(|e| DocError::PathRead {
            pb: pb.clone(),
            abs: abs.clone(),
            original: e,
        })?;
        let items = DocSourceItems::from_str(&file_str)?;
        let new_self = Self {
            file_content: file_str,
            doc_src_items: items,
        };
        Ok(new_self)
    }
}

impl FromStr for DocSource {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = DocSourceItems::from_str(&s)?;
        Ok(Self {
            file_content: s.to_string(),
            doc_src_items: items,
        })
    }
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
}
