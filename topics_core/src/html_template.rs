use crate::context::Context;

pub trait HtmlTemplate {
    fn template(&self, ctx: &Context) -> anyhow::Result<String>;
}
