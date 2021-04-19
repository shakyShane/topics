use std::collections::HashMap;
use std::path::PathBuf;

use crate::db_error::SerializedError;
use crate::doc_src::MdDocSource;
use crate::items::Item;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, TypeScriptify, serde::Serialize)]
pub struct Output {
    pub docs: HashMap<PathBuf, MdDocSource>,
    pub items: Vec<Item>,
    pub errors: Vec<SerializedError>,
}
