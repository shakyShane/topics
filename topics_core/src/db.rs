use crate::doc::Doc;
use crate::items::{Item, ItemWrap};
use std::collections::{HashMap, HashSet};

type ItemGraph = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub struct Db {
    items: ItemGraph,
    pub items_2: HashMap<String, Item>,
}

impl Db {
    pub fn try_from_docs<'a>(docs: &Vec<Doc>) -> anyhow::Result<Self> {
        let mut graph: ItemGraph = HashMap::new();
        let mut items_2: HashMap<String, Item> = HashMap::new();
        for doc in docs {
            for (topic_name, topic) in &doc.topics {
                let entry = graph.entry(topic_name.clone()).or_insert(HashSet::new());
                items_2.insert(topic_name.to_owned(), Item::Topic(topic.clone()));
                for dep in &topic.deps {
                    match dep {
                        ItemWrap::Named(item_name) => {
                            entry.insert(item_name.clone());
                        }
                        ItemWrap::Item(_) => todo!("Item::Item not ready yet"),
                    }
                }
                for dep in &topic.steps {
                    match dep {
                        ItemWrap::Named(item_name) => {
                            entry.insert(item_name.clone());
                        }
                        ItemWrap::Item(_) => todo!("Item::Item not ready yet"),
                    }
                }
            }
            for (name, _topic) in &doc.commands {
                graph.entry(name.clone()).or_insert(HashSet::new());
            }
            for (name, _topic) in &doc.dep_checks {
                graph.entry(name.clone()).or_insert(HashSet::new());
            }
            for (name, _topic) in &doc.instructions {
                graph.entry(name.clone()).or_insert(HashSet::new());
            }
        }
        let _ = detect_cycle(&graph)?;
        Ok(Self {
            items: graph,
            items_2,
        })
    }
}

fn detect_cycle(graph: &ItemGraph) -> anyhow::Result<()> {
    for (name, hash_set) in graph {
        for child_name in hash_set {
            if let Some(child_set) = graph.get(child_name) {
                if child_set.contains(name) {
                    return Err(anyhow::anyhow!(
                        "Infinite loop detected, please check usages of `{}`",
                        name
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::context::Context;

    use std::path::PathBuf;

    #[test]
    fn test() -> anyhow::Result<()> {
        let ctx = Context::default();
        let doc1 = PathBuf::from("../fixtures/graph/commands.yaml");
        let doc2 = PathBuf::from("../fixtures/graph/deps.yaml");
        let doc3 = PathBuf::from("../fixtures/graph/topics.yaml");
        let docs = vec![doc1, doc2, doc3];
        let good = ctx.read_docs_unwrapped(&docs);
        assert_eq!(good.len(), 3);
        let _ = Db::try_from_docs(&good)?;
        Ok(())
    }

    #[test]
    fn test_cycle_detection() -> anyhow::Result<()> {
        let ctx = Context::default();
        let doc1 = PathBuf::from("../fixtures/cycle/topics.yaml");
        let docs = vec![doc1];
        let good = ctx.read_docs_unwrapped(&docs);
        assert_eq!(good.len(), 1);
        let res = Db::try_from_docs(&good);
        eprintln!("{:?}", res);
        assert!(res.is_err());
        Ok(())
    }
}
