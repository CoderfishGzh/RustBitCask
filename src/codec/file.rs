use std::{io::BufWriter, fs::File};



pub struct WriteInfo {
    pos: u128,
    writer: BufWriter<File>,
}