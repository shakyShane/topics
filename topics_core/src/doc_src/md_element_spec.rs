use crate::doc_src::{collect_single_line_text, to_items, MdDocSource, MdElements, MdSrc};
use crate::items::{Command, Instruction, Item};
use comrak::nodes::{Ast, NodeValue};
use comrak::{format_html, Arena, ComrakOptions};
use std::convert::TryInto;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn test_command() -> anyhow::Result<()> {
    let input = r#"# Command: Run unit tests

A block of text following the title!

```shell command --cwd="./"
echo hello world!
```
        "#;
    let mut md_src = MdSrc::new(MdDocSource::from_str(input)?);
    let elems = md_src.parse().as_ref().expect("parse md");
    let items = elems.as_items()?;
    let first = items.get(0).expect("at least 1 item");
    if let Item::Command(Command {
        name,
        command,
        ast_range,
        ..
    }) = first
    {
        assert_eq!(name, &String::from("Run unit tests"));
        assert_eq!(command, &String::from("echo hello world!"));
        let html = elems.as_html(ast_range);
        assert_eq!(
            r#"<pre><code class="language-shell">echo hello world!
</code></pre>
"#,
            html
        )
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

#[test]
fn test_steps() -> anyhow::Result<()> {
    let command_input = r#"# Command: Run unit tests

A block of text following the title!

```shell command --cwd="./"
echo hello world!
```
        "#;
    let mut md_src = MdSrc::from_str(command_input)?;
    md_src.doc_src.input_file = Some(PathBuf::from("./command.md"));

    let topic_input = r#"# Topic: Run all tests

## Dependencies

- Access to Azure

## Steps

- Run unit tests
- So something else
        "#;
    let mut md_src_topic = MdSrc::from_str(topic_input)?;
    let elems = md_src_topic.parse().as_ref().expect("parse md");
    let items = elems.as_items()?;
    Ok(())
}

#[test]
fn test_dep_check() -> anyhow::Result<()> {
    let input = r#"
# Dependency Check: Node JS installed globally

Node JS is required and should be on version 12.0

```shell verify
node -v
```
"#;
    let mut md_src = MdSrc::from_str(input)?;
    let elems = md_src.parse().as_ref().expect("parse md");
    let items = elems.as_items()?;
    dbg!(items);
    Ok(())
}
