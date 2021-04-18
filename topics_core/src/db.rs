use crate::db_error::{CycleError, DbError, ErrorRef, IntoDbError};
use crate::doc::Doc;
use crate::doc_src::{DocSource, MdSrc};
use crate::items::{marker_ref, name_ref, Item, ItemWrap};
use std::collections::{HashMap, HashSet};

type ItemGraph = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub struct Db {
    graph: ItemGraph,
    // pub item_map: HashMap<String, ItemTracked>,
}

impl Db {
    pub fn try_from_docs(docs: &[Doc]) -> anyhow::Result<Self> {
        let graph: ItemGraph = HashMap::new();
        let mut src_items: Vec<MdSrc> = vec![];

        for doc in docs {
            if let DocSource::Md(md) = &doc.source {
                for item in &md.doc_src_items.items {
                    let next = MdSrc::new(md, item);
                    src_items.push(next);
                }
            }
        }

        for item in src_items.iter() {
            item.parse();
        }

        let mut hm: HashMap<&'_ String, Vec<&'_ String>> = HashMap::new();

        let items: Vec<(&'_ MdSrc, Vec<Item>)> = src_items
            .iter()
            .map(|src| {
                (
                    src,
                    src.md_elements
                        .borrow()
                        .as_ref()
                        .expect("unwrap")
                        .as_items(),
                )
            })
            .collect();

        let mut item_lookup: HashMap<&'_ String, (&'_ MdSrc, &'_ Item)> = HashMap::new();

        for (mdsrc, items) in &items {
            for item in items {
                item_lookup.insert(name_ref(item), (mdsrc, item));
            }
        }

        for (_mdsrc, items) in &items {
            for item in items {
                let lm = marker_ref(item);
                let entry = hm.entry(&lm.item).or_insert(Vec::new());
                if let Item::Topic(topic) = item {
                    for named_ref in topic.deps.iter().chain(topic.steps.iter()) {
                        match named_ref {
                            ItemWrap::NamedRef(line_marker) => {
                                entry.push(&line_marker.item);
                            }
                            ItemWrap::Item(_) => todo!("inline item"),
                        }
                    }
                }
            }
        }
        // dbg!(hm);
        let cycles = detect_cycle(&hm, &item_lookup);

        cycles.iter().for_each(|(cyc, (_mdsrc, _item))| {
            println!("{}", cyc);
        });
        // dbg!(cycles);

        Ok(Self { graph })
    }

    #[cfg(test)]
    pub fn unknown(&self) -> HashMap<String, HashSet<String>> {
        let output: HashMap<String, HashSet<String>> = HashMap::new();
        for (_parent_name, hash_set) in &self.graph {
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
        let output = vec![];
        for _parent_name in self.graph.keys() {
            let _used = false;
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

fn detect_cycle<'a>(
    graph: &'a HashMap<&'a String, Vec<&'a String>>,
    lookup: &'a HashMap<&'a String, (&'a MdSrc<'a>, &'a Item)>,
) -> Vec<(DbError<'a>, (&'a MdSrc<'a>, &'a Item))> {
    let mut output: Vec<(DbError, (&MdSrc, &Item))> = vec![];
    for (parent_name, list_of_names) in graph {
        for child_name in list_of_names {
            if let Some(child_list) = graph.get(child_name) {
                if child_list.contains(parent_name) {
                    if let Some((src, item)) = lookup.get(parent_name) {
                        let cycle_err = CycleError::new(*parent_name, *child_name);
                        let db_err = cycle_err.into_db_error(src, item);
                        output.push((db_err, (src, item)));
                    }
                }
            }
        }
    }
    output
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
            PathBuf::from("../fixtures/md/topics_2.md"),
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
        let _unknown = g.unknown();
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
