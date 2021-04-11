use crate::doc_src::MdElements;
use std::str::FromStr;

#[test]
fn test_command() -> anyhow::Result<()> {
    let input = r#"
# Command: Run unit tests

```shell command --cwd="./"
echo hello world!
```
        "#;
    let md_elements = MdElements::from_str(input)?;
    // assert_eq!(
    //     md_elements.elements.get(0).unwrap(),
    //     &Element::h1("Command: Run unit tests")
    // );
    // assert_eq!(
    //     md_elements.elements.get(1).unwrap(),
    //     &Element::code_block("echo hello world!", Some(r#"shell command --cwd="./""#))
    // );
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
    println!("{:#?}", md_elements.items.get(0).unwrap());
    // assert_eq!(
    //     md_elements.elements.get(0).unwrap(),
    //     &Element::h1("Command: Run unit tests")
    // );
    // assert_eq!(
    //     md_elements.elements.get(1).unwrap(),
    //     &Element::code_block("echo hello world!", Some(r#"shell command --cwd="./""#))
    // );
    Ok(())
}
