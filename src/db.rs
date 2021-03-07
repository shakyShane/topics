use crate::doc::Doc;
use crate::items::Item;
use std::collections::HashMap;

struct Db<'a> {
    items: HashMap<String, MappedItem<'a>>,
}

struct MappedItem<'a> {
    src: &'a Doc,
    inner: &'a Item,
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test() -> anyhow::Result<()> {
        let _pb = std::path::PathBuf::from("/input-yaml.yml");
        let input = r#"
kind: Topic
name: Run screen shot tests
deps: 
    - setup
steps: 
    - oops
    
---

kind: Command
name: setup
cwd: .
command: echo hello world

---


kind: Instruction
cwd: .
command: echo hello world
"#;
        let doc = Doc::from_str(input);
        dbg!(doc);
        Ok(())
    }
}
