// use topics_core::output::TypeScriptifyTrait;
use multi_doc::{MultiDoc, SingleDoc};
use topics_core::cwd::Cwd;
use topics_core::doc_src::MdDocSource;
use topics_core::items::{
    Command, DependencyCheck, Env, FileExistsCheck, HostEntriesCheck, HostEntry, Instruction, Item,
    ItemWrap, LineMarker, TaskGroup, Topic,
};
use topics_core::{CycleError, Output, SerializedError};
use typescript_definitions::TypeScriptifyTrait;

fn main() {
    println!("{}", Output::type_script_ify());
    println!("{}", MdDocSource::type_script_ify());
    println!("{}", LineMarker::<String>::type_script_ify());
    println!("{}", Item::type_script_ify());
    println!("{}", ItemWrap::type_script_ify());
    println!("{}", Command::type_script_ify());
    println!("{}", Instruction::type_script_ify());
    println!("{}", Topic::type_script_ify());
    println!("{}", FileExistsCheck::type_script_ify());
    println!("{}", SerializedError::type_script_ify());
    println!("{}", MultiDoc::type_script_ify());
    println!("{}", SingleDoc::type_script_ify());
    println!("{}", CycleError::type_script_ify());
    println!("{}", DependencyCheck::type_script_ify());
    println!("{}", HostEntriesCheck::type_script_ify());
    println!("{}", HostEntry::type_script_ify());
    println!("{}", TaskGroup::type_script_ify());
    println!("{}", Env::type_script_ify());
    println!("{}", Cwd::type_script_ify());
}
