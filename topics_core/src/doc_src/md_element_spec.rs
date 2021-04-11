use crate::doc_src::MdElements;
use crate::items::{Command, Instruction, Item};
use std::str::FromStr;

#[test]
fn test_command() -> anyhow::Result<()> {
    let input = r#"
# Command: Run unit tests

A block of text following the title!

```shell command --cwd="./"
echo hello world!
```
        "#;
    let md_elements = MdElements::from_str(input)?;
    let first = md_elements.items.get(0);
    if let Some(Item::Command(Command { name, .. })) = first {
    } else {
        unreachable!();
    }
    Ok(())
}

#[test]
fn test_instruction() -> anyhow::Result<()> {
    let input = r#"
# Instruction: Call IT help desk

Lorem ipsum dolor sit *amet*, consectetur adipisicing elit. Accusamus assumenda molestiae natus quaerat rerum sed suscipit tenetur? Aperiam minima, quos. Commodi corporis cupiditate facilis in minus, quae quis quos similique!

More stuff here

<h1>Oops!</h1>

```shell
oh feck
```
        "#;
    let md_elements = MdElements::from_str(input)?;
    let first = md_elements.items.get(0);
    if let Some(Item::Instruction(Instruction { name, ast })) = first {
        assert_eq!(name, &String::from("Call IT help desk"));
        assert_eq!(ast.len(), 5);
    } else {
        unreachable!();
    }
    Ok(())
}
