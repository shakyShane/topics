use crate::cwd::Cwd;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Command {
    pub cwd: Cwd,
    pub command: String,
    pub name: String,
    pub env: Option<Env>,
}

#[derive(Debug, structopt::StructOpt)]
pub struct CommandInlineArgs {
    #[structopt(long, default_value = "./")]
    pub cwd: Cwd,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Env {
    pub values: Option<HashMap<String, String>>,
}

impl FromStr for Env {
    type Err = anyhow::Error;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self { values: None })
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            command: "echo 'hello world'".to_string(),
            name: "run unit tests command".to_string(),
            env: Default::default(),
        }
    }
}
