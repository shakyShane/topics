use std::env::current_dir;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Cwd(pub PathBuf);

impl FromStr for Cwd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from(s)))
    }
}

impl Display for Cwd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Default for Cwd {
    fn default() -> Self {
        Self(current_dir().expect("can access cwd"))
    }
}

impl Cwd {
    pub fn join_path(&self, pb: impl Into<PathBuf>) -> PathBuf {
        self.0.join(pb.into())
    }
}
