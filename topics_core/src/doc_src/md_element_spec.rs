// #[test]
// fn test_command() -> anyhow::Result<()> {
//     let input = r#"# Command: Run unit tests
//
// A block of text following the title!
//
// ```shell command --cwd="./"
// echo hello world!
// ```
//         "#;
//     let ds = MdDocSource::from_str(input)?;
//     let first = ds.doc_src_items.items.get(0).expect("first item");
//     let md_src = MdSrc::new(&ds, first);
//     md_src.parse();
//     let items = md_src.items.borrow();
//     let items = items.as_ref().expect("items");
//     let first = items.get(0).expect("at least 1 item");
//     if let Item::Command(Command {
//         name,
//         command,
//         ast_range,
//         ..
//     }) = first
//     {
//         assert_eq!(name, &String::from("Run unit tests"));
//         assert_eq!(command, &String::from("echo hello world!"));
//         let html = md_src
//             .md_elements
//             .borrow()
//             .as_ref()
//             .expect("Oops!")
//             .as_html(ast_range);
//         assert_eq!(
//             r#"<pre><code class="language-shell">echo hello world!
// </code></pre>
// "#,
//             html
//         )
//     } else {
//         unreachable!();
//     }
//     Ok(())
// }

// #[test]
// fn test_instruction() -> anyhow::Result<()> {
//     let input = r#"
// # Instruction: Call IT help desk
//
// Lorem ipsum dolor sit *amet*, consectetur adipisicing elit. Accusamus assumenda molestiae natus quaerat rerum sed suscipit tenetur? Aperiam minima, quos. Commodi corporis cupiditate facilis in minus, quae quis quos similique!
//
// More stuff here
//
// <h1>Oops!</h1>
//
// ```shell
// oh feck
// ```
//         "#;
//     let arena = Arena::new();
//     let md = MdElements::new(input, &arena);
//     let items: Vec<Item> = md.as_items();
//     let first = items.get(0);
//     if let Some(Item::Instruction(inst)) = first {
//         assert_eq!(inst.name, String::from("Call IT help desk"));
//     } else {
//         unreachable!();
//     }
//     Ok(())
// }
//
// #[test]
// fn test_steps() -> anyhow::Result<()> {
//     //     let command_input = r#"# Command: Run unit tests
//     //
//     // A block of text following the title!
//     //
//     // ```shell command --cwd="./"
//     // echo hello world!
//     // ```
//     //         "#;
//     // let mut md_src = MdSrc::new();
//     // md_src.doc_src.input_file = Some(PathBuf::from("./command.md"));
//
//     let topic_input = r#"# Topic: Run all tests
//
// ## Dependencies
//
// - Access to Azure
//
// ## Steps
//
// - Run unit tests
// - So something else
//         "#;
//     let md_src_topic = MdSrc::new();
//     let elems = md_src_topic.parse(topic_input);
//     let items = elems.items();
//     dbg!(items);
//     Ok(())
// }
//
// #[test]
// fn test_dep_check() -> anyhow::Result<()> {
//     let input = r#"
// # Dependency Check: Node JS installed globally
//
// Node JS is required and should be on version 12.0
//
// ```shell verify
// node -d
// ```
//
// "#;
//     let md_src = MdSrc::new();
//     let elems = md_src.parse(input);
//     let items = elems.items();
//     dbg!(items);
//     Ok(())
// }
