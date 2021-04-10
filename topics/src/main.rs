use comrak::arena_tree::Node;
use comrak::nodes::{Ast, ListType};
use topics_core::from_cli;

fn main() {
    // std::env::set_var("RUST_LOG", "topics=trace");
    env_logger::init();
    std::process::exit(match from_cli() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}

#[test]
fn test_parse() {
    use pest::Parser;
    let input = r#"# Action: view pods `logs`

## Dependencies

- step 3
    hello there
    ```shell command
    oops!
    ```
- step 4
"#;
    use comrak::nodes::{AstNode, NodeValue};
    use comrak::{format_html, parse_document, Arena, ComrakOptions};

    fn collect_single_line_text<'a>(node: &'a AstNode<'a>) -> String {
        node.children()
            .filter_map(|n| match &n.data.borrow().value {
                NodeValue::Text(t) => Some(std::str::from_utf8(t).unwrap().to_string()),
                // todo, preserve this information?
                NodeValue::Code(t) => Some(std::str::from_utf8(t).unwrap().to_string()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("")
    }

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let root = parse_document(&arena, input, &ComrakOptions::default());
    fn iter_nodes<'a>(node: &'a AstNode<'a>) {
        // dbg!(node.first_child());
        let n = node.data.borrow();
        if let NodeValue::Heading(heading) = n.value {
            if heading.level == 1 {
                println!("heading 1, line=[--{}--]", n.start_line);
                let t = collect_single_line_text(node);
                println!("heading 1={}", t);
            }
            if heading.level == 2 {
                let t = collect_single_line_text(node);
                if t == "Dependencies" {
                    println!("deps!");
                }
                if t == "Steps" {
                    println!("steps!");
                }
            }
            if heading.level == 3 {
                println!("heading 3");
            }
        } else if let NodeValue::List(node_list) = n.value {
            println!("list start -> {}", n.start_line);
            for item_child in node.children() {
                let list_item = item_child.data.borrow();
                println!("\tlist item start line -> {}", list_item.start_line);
                if let NodeValue::Item(node_list) = &list_item.value {
                    for c in item_child.children() {
                        if let Some(first) = c.first_child() {
                            let node = first.data.borrow();
                            if let NodeValue::Paragraph = &node.value {
                                let text = collect_single_line_text(first);
                                println!("text=||--{}--||", text);
                            }
                        }
                    }
                }
            }
        } else {
            for c in node.children() {
                //     let b = c.data.borrow();
                //     if let NodeValue::List(node_list) = &b.value {
                //         println!("++node_list.start=[{}]", b.start_line);
                //         iter_nodes(c);
                //         println!("--node_list.end=[{}]", b.start_line);
                //     }
                //     if let NodeValue::Text(t) = &b.value {
                //         println!();
                //         println!("\tt-->|{}|", std::str::from_utf8(&t).unwrap());
                //         println!();
                //     }
                //     if let NodeValue::Heading(heading) = &b.value {
                //         println!("heading.h{}.line=[{}]", heading.level, b.start_line);
                //         iter_nodes(c);
                //         println!("heading.h{}.end=[{}]", heading.level, b.start_line);
                //     }
                iter_nodes(c)
            }
        }
    }

    iter_nodes(root);
}
