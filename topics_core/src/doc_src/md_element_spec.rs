use crate::doc_src::{collect_single_line_text, to_items, MdElements};
use crate::items::{Command, Instruction, Item};
use comrak::nodes::{Ast, NodeValue};
use comrak::{format_html, Arena, ComrakOptions};
use std::convert::TryInto;
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
    let arena = Arena::new();
    let md_elements = MdElements::new(input, &arena);
    let first = md_elements.items.get(0);
    if let Some(Item::Command(Command { name, .. })) = first {
        assert_eq!(name, &String::from("Run unit tests"));
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
    let arena = Arena::new();
    let md = MdElements::new(input, &arena);
    let items: Vec<Item> = md.as_items()?;
    let first = items.get(0);
    if let Some(Item::Instruction(inst)) = first {
        assert_eq!(inst.name, String::from("Call IT help desk"));
    } else {
        unreachable!();
    }
    Ok(())
}
