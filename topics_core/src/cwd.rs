use std::env::current_dir;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use typescript_definitions::TypeScriptify;

///
/// A wrapper type to allow a default implementation
/// for the current working directory
///
/// # Example
///
/// You can initialize this struct with a preset path
///
/// ```rust
/// # use topics_core::cwd::Cwd;
/// # use std::str::FromStr;
/// # use std::path::PathBuf;
/// let cwd = Cwd::from_str("/root").unwrap();
/// let joined = cwd.join("dir");
/// assert_eq!(joined, PathBuf::from("/root/dir") );
/// ```
/// # Example
///
/// It's more useful though because it implements default
/// using the current program directory.
///
/// ```rust
/// # use topics_core::cwd::Cwd;
/// # use std::env::current_dir;
/// let cwd = Cwd::default().join("there");
/// let this_dir = current_dir().unwrap().join("there");
/// assert_eq!(cwd, this_dir)
/// ```
///
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, TypeScriptify,
)]
pub struct Cwd(pub PathBuf);

impl Deref for Cwd {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
