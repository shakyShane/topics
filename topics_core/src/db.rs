use crate::doc::{Doc, ItemTracked};
use crate::items::{Item, ItemWrap};
use std::collections::{HashMap, HashSet};

type ItemGraph = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub struct Db {
    graph: ItemGraph,
    pub item_map: HashMap<String, ItemTracked>,
}

impl Db {
    pub fn try_from_docs<'a>(docs: &Vec<Doc>) -> anyhow::Result<Self> {
        let mut graph: ItemGraph = HashMap::new();
        let mut item_map: HashMap<String, ItemTracked> = HashMap::new();
        for doc in docs {
            for item_tracked in &doc.items {
                let this_item = &item_tracked.item;
                let entry = graph.entry(this_item.name()).or_insert(HashSet::new());
                item_map.insert(this_item.name(), (*item_tracked).clone());
                match this_item {
                    Item::Topic(topic) => {
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
                    _ => {
                        // println!("nothing else to do")
                    }
                }
            }
        }
        let _ = detect_cycle(&graph)?;
        Ok(Self { graph, item_map })
    }
    pub fn unknown(&self) -> Vec<String> {
        let mut output = vec![];
        for (_, hash_set) in &self.graph {
            for child_name in hash_set {
                if let None = self.graph.get(child_name) {
                    output.push(child_name.clone())
                }
            }
        }
        output
    }
    pub fn unused(&self) -> Vec<String> {
        let mut output = vec![];
        for (parent_name, _) in &self.graph {
            let mut used = false;
            let item = self.item_map.get(parent_name);
            if let Some(ItemTracked {
                item: Item::Topic(..),
                ..
            }) = item
            {
                used = true
            }
            for (_, child_hash_set) in &self.graph {
                if child_hash_set.contains(&parent_name.clone()) {
                    used = true
                }
            }
            if !used {
                output.push(parent_name.clone())
            }
        }
        output
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

    fn graph_db() -> Db {
        let ctx = Context::default();
        let doc1 = PathBuf::from("../fixtures/graph/commands.yaml");
        let doc2 = PathBuf::from("../fixtures/graph/deps.yaml");
        let doc3 = PathBuf::from("../fixtures/graph/topics.yaml");
        let docs = vec![doc1, doc2, doc3];
        let good = ctx.read_docs_unwrapped(&docs);
        assert_eq!(good.len(), 3);
        Db::try_from_docs(&good).expect("test data")
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

    #[test]
    fn test_detect_unknown() -> anyhow::Result<()> {
        let g = graph_db();
        let unknown = g.unknown();
        assert_eq!(unknown, vec![String::from("install helm")]);
        Ok(())
    }

    #[test]
    fn test_detect_unused() -> anyhow::Result<()> {
        let g = graph_db();
        let unknown = g.unused();
        assert_eq!(unknown, vec![String::from("unused command here")]);
        Ok(())
    }
}
