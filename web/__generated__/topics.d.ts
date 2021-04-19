export type Output = {     docs: { [key: string]: MdDocSource }; items: Item []; errors:     SerializedError [] };
export type MdDocSource = {     input_file: string | null; file_content: string; doc_src_items:     MultiDoc };
export type LineMarker<T> = { line_start: number | null; item: T };
export type Item = 
 | { kind: "Command"; content: Command } 
 | { kind: "FileExistsCheck"; content: FileExistsCheck } 
 | { kind: "DependencyCheck"; content: DependencyCheck } 
 | { kind: "Instruction"; content: Instruction } 
 | { kind: "HostEntriesCheck"; content: HostEntriesCheck } 
 | { kind: "Topic"; content: Topic } 
 | { kind: "TaskGroup"; content: TaskGroup };
export type ItemWrap = 
 | { kind: "NamedRef"; content: LineMarker<string>} 
 | { kind: "Item"; content: Item };
export type Command = {     name: LineMarker<string>; cwd: Cwd; command: string; env: Env |     null };
export type Instruction = { name: LineMarker<string>};
export type Topic = { name: LineMarker<string>; steps: ItemWrap []; deps: ItemWrap [] };
export type FileExistsCheck = { cwd: string; path: string; name: string };
export type SerializedError = 
 | { kind: "Cycle"; content: CycleError };
export type MultiDoc = { items: SingleDoc [] };
export type SingleDoc = { line_start: number; line_end: number; content: string };
export type CycleError = { from: string; to: LineMarker<string>};
export type DependencyCheck = {     name: LineMarker<string>; verify: string; autofix: string | null;     url: string | null };
export type HostEntriesCheck = { hosts: HostEntry []; name: string };
export type HostEntry = { domain: string };
export type TaskGroup = { name: string; steps: ItemWrap [] };
export type Env = { values: { [key: string]: string } | null };
// A wrapper type to allow a default implementation
// for the current working directory
// # Example
// You can initialize this struct with a preset path
// ```rust
// # use topics_core::cwd::Cwd;
// # use std::str::FromStr;
// # use std::path::PathBuf;
// let cwd = Cwd::from_str("/root").unwrap();
// let joined = cwd.join_path("dir");
// assert_eq!(joined, PathBuf::from("/root/dir") );
// ```
// # Example
// It's more useful though because it implements default
// using the current program directory.
// ```rust
// # use topics_core::cwd::Cwd;
// # use std::env::current_dir;
// let cwd = Cwd::default().join_path("there");
// let this_dir = current_dir().unwrap().join("there");
// assert_eq!(cwd, this_dir)
// ```
export type Cwd = string;
