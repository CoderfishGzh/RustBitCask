use std::path::{Path, PathBuf};

pub struct Entry {
    file_id: PathBuf,
    value_sz: u128,
    value_pos: u128,
    tstamp: u128,
}
