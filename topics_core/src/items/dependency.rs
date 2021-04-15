use crate::cwd::Cwd;
use crate::doc_src::code_fence;
use crate::items::LineMarker;

#[derive(Debug, Clone)]
pub struct DependencyCheck {
    pub name: LineMarker<String>,
    pub verify: String,
    pub autofix: Option<String>,
    pub url: Option<String>,
}

impl DependencyCheck {
    pub fn minimal(name: &str, verify: &str) -> Self {
        Self {
            verify: name.to_string(),
            name: LineMarker::new(verify.to_string(), None),
            autofix: None,
            url: None,
        }
    }
}

impl Default for DependencyCheck {
    fn default() -> Self {
        Self {
            verify: "echo 'verify script missing'; exit 1;".to_string(),
            name: LineMarker::new("install node".to_string(), None),
            autofix: None,
            url: Some("https://nodejs.org".to_string()),
        }
    }
}

impl DependencyCheck {
    pub fn with_content(&mut self, content: &str, params: &str) {
        match code_fence::parse_code_fence_args(params) {
            Ok(Some(code_fence::Cmd::Verify(_))) => {
                self.verify = content.to_string();
            }
            Ok(Some(code_fence::Cmd::AutoFix(_))) => {
                self.autofix = Some(content.to_string());
            }
            _a => {
                todo!("what to do when inline args are invalid")
            }
        }
    }
}

#[derive(Debug, structopt::StructOpt)]
pub struct VerifyInlineArgs {
    #[structopt(long, default_value = "./")]
    pub cwd: Cwd,
}

#[derive(Debug, structopt::StructOpt)]
pub struct AutoFixInlineArgs {
    #[structopt(long, default_value = "./")]
    pub cwd: Cwd,
}
