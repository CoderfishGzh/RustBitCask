pub(crate) use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;

use crate::codec::entry::Entry;
use crate::Result;
use std::io::{BufReader, BufWriter};
use crate::codec::file::WriteInfo;

pub struct Store {
    // curr file
    curr_file: WriteInfo,
    curr_id: u128,
    // read-only file
    immufile: HashMap<u128, BufReader<File>>,
    // indexs for entry
    indexs: HashMap<Vec<u8>, Entry>,
}

impl Store {
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!()
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }

    pub fn new() -> Self {
        todo!()
    }
}
