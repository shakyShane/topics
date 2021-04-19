use std::collections::HashMap;

use std::str::FromStr;

use crate::cwd::Cwd;
use crate::doc_src::ast_range::AstRange;
use crate::doc_src::code_fence;
use crate::items::LineMarker;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, serde::Serialize, TypeScriptify)]
pub struct Command {
    pub name: LineMarker<String>,
    pub cwd: Cwd,
    pub command: String,
    pub env: Option<Env>,
    #[serde(skip)]
    pub ast_range: AstRange,
}

#[derive(Debug, structopt::StructOpt)]
pub struct CommandInlineArgs {
    #[structopt(long, default_value = "./")]
    pub cwd: Cwd,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, TypeScriptify)]
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
            command: "echo 'no command'; exit 1; ".to_string(),
            name: LineMarker::new(String::new(), None),
            env: Default::default(),
            ast_range: Default::default(),
        }
    }
}

impl Command {
    pub fn with_content(&mut self, content: &str) {
        self.command = content.to_string();
    }
    pub fn with_cli_params(&mut self, params: &str) {
        match code_fence::parse_code_fence_args(params) {
            Ok(Some(code_fence::Cmd::Command(inner))) => {
                // we only assign this code block if it has ```shell command ...
                self.cwd = inner.cwd;
            }
            _a => {
                // todo!("handle parsing code-block inline args")
            }
        }
    }
}
