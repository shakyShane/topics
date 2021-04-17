use crate::doc::{Doc, ItemTracked};
use crate::doc_src::{DocSource, MdSrc};
use crate::items::{Item, ItemWrap, LineMarker};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type ItemGraph = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub struct Db {
    graph: ItemGraph,
    // pub item_map: HashMap<String, ItemTracked>,
}

impl Db {
    pub fn try_from_docs(docs: &[Doc]) -> anyhow::Result<Self> {
        let mut graph: ItemGraph = HashMap::new();
        // let mut item_map: HashMap<String, ItemTracked> = HashMap::new();
        let mut items: Vec<MdSrc> = vec![];
        for doc in docs {
            if let DocSource::Md(md) = &doc.source {
                for item in &md.doc_src_items.items {
                    let next = MdSrc::new(&md);
                    items.push(next);
                    // items.push(MdSrc::new().parse(item.content.as_str()));
                    // let md_src = MdSrc::new().parse(item.content.as_str());
                    // if let Some(elems) = md_src.parse(item.content.as_str()) {
                    //     let items = elems.as_items();
                    //     let html = elems.as_html((vec![0], 1));
                    //     println!("html={}", html);
                    //     if let Ok(items) = items {
                    //         for item in items {
                    //             let entry = graph.entry(item.name()).or_insert_with(HashSet::new);
                    //             match &item {
                    //                 Item::Topic(topic) => {
                    //                     for dep in topic.deps.iter().chain(topic.steps.iter()) {
                    //                         match dep {
                    //                             ItemWrap::NamedRef(line_marker) => {
                    //                                 entry.insert(line_marker.to_string());
                    //                             }
                    //                             ItemWrap::Item(_) => todo!("inline item"),
                    //                         }
                    //                     }
                    //                 }
                    //                 Item::Command(_) => {}
                    //                 Item::FileExistsCheck(_) => {}
                    //                 Item::DependencyCheck(_) => {}
                    //                 Item::Instruction(_) => {}
                    //                 Item::HostEntriesCheck(_) => {}
                    //                 Item::TaskGroup(_) => {}
                    //             }
                    //             println!("name->{}", item.name());
                    //         }
                    //     }
                    // }
                }
            }
        }
        for item in items.iter() {
            item.parse("# heading");
        }
        // let _ = detect_cycle(&graph)?;
        Ok(Self { graph })
    }
    #[cfg(test)]
    pub fn unknown(&self) -> HashMap<String, HashSet<String>> {
        let mut output: HashMap<String, HashSet<String>> = HashMap::new();
        for (parent_name, hash_set) in &self.graph {
            for child_name in hash_set {
                if self.graph.get(child_name).is_none() {
                    // let _matched_item = self.item_map.get(parent_name);
                    // let entry = output
                    //     .entry(parent_name.clone())
                    //     .or_insert_with(HashSet::new);
                    // entry.insert(child_name.clone());
                }
            }
        }
        output
    }
    #[cfg(test)]
    pub fn unused(&self) -> Vec<String> {
        let mut output = vec![];
        for parent_name in self.graph.keys() {
            let mut used = false;
            // let item = self.item_map.get(parent_name);
            // if let Some(ItemTracked {
            //     item: Item::Topic(..),
            //     ..
            // }) = item
            // {
            //     used = true
            // }
            // for child_hash_set in self.graph.values() {
            //     if child_hash_set.contains(&parent_name.clone()) {
            //         used = true
            //     }
            // }
            // if !used {
            //     output.push(parent_name.clone())
            // }
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
    fn test_doc_from_src() {
        let ctx = Context::default();
        let f = ctx.read_docs_unwrapped(&vec![
            PathBuf::from("../fixtures/md/topics.md"),
            PathBuf::from("../fixtures/md/commands.md"),
        ]);
        let db = Db::try_from_docs(&f);
        dbg!(db);
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
        // for (key, _value) in unknown {
        //     let parent = g.item_map.get(&key);
        //     if let Some(ItemTracked { item, .. }) = parent {
        //         println!("missing item in {}", item.name())
        //     }
        // }
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
