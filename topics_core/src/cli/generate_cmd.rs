use crate::cli::{SubCommand, SubCommandResult};
use crate::context::Context;

use crate::items::{DependencyCheck, Item};

#[derive(Debug, Clone, structopt::StructOpt)]
#[structopt(alias = "g")]
pub struct GenerateCmd {
    #[structopt(name = "items")]
    items: Vec<Item>,
}

impl SubCommand for GenerateCmd {
    fn exec(&self, _ctx: &Context) -> SubCommandResult<()> {
        if self.items.is_empty() {
            let items = vec![
                Item::Topic(Default::default()),
                Item::TaskGroup(Default::default()),
                Item::DependencyCheck(Default::default()),
                Item::DependencyCheck(DependencyCheck::minimal("install yarn", "yarn -v")),
                Item::Command(Default::default()),
                Item::Instruction(Default::default()),
            ];
            print_items(&items);
        } else {
            print_items(&self.items);
        }
        Ok(())
    }
}

fn print_items(items: &[Item]) {
    for item in items {
        let yaml = serde_yaml::to_string(&item);
        println!("{}", yaml.unwrap());
    }
}
