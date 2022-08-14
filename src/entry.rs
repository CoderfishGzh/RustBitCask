pub(crate) const ENTRY_HEADER_SIZE: u32 = 10;
pub const PUT: u16 = 1;

// the data structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub key: Vec<u8>,

    pub value: Vec<u8>,

    pub key_size: u32,

    pub value_size: u32,

    pub mark: u16,
}

impl Entry {
    // Create the entry from the key and value
    pub(crate) fn new_entry(key: Vec<u8>, value: Vec<u8>, mark: u16) -> Entry {
        Entry {
            key: key.clone(),
            value: value.clone(),
            key_size: key.len() as u32,
            value_size: value.len() as u32,
            mark,
        }
    }

    // Return the size of this entry
    pub fn get_size(&self) -> u32 {
        ENTRY_HEADER_SIZE + self.key_size + self.value_size
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut entry_bytes: Vec<u8> = Vec::new();
        entry_bytes.extend(self.key_size.to_be_bytes());
        entry_bytes.extend(self.value_size.to_be_bytes());
        entry_bytes.extend(self.mark.to_be_bytes());
        entry_bytes.extend(self.key.clone());
        entry_bytes.extend(self.value.clone());
        entry_bytes
    }

    pub fn decode(buf: Vec<u8>) -> Self {
        let ks = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let vs = u32::from_be_bytes(buf[4..8].try_into().unwrap());
        let mark = u16::from_be_bytes(buf[8..10].try_into().unwrap());

        Self {
            key: vec![],
            value: vec![],
            key_size: ks,
            value_size: vs,
            mark,
        }
    }
}
