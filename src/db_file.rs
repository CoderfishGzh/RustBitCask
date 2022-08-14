use crate::entry::{Entry, ENTRY_HEADER_SIZE};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;

const FILE_NAME: &str = "db.data";

pub struct DBFile {
    pub file: File,
    pub offset: u64,
}

impl DBFile {
    pub fn read(&self, offset: u64) -> Result<Entry, io::Error> {
        // 创建一个固定长度的缓冲区
        let mut buf = [0; ENTRY_HEADER_SIZE as usize];

        // 读取文件到缓冲区
        if let Err(e) = self.file.read_at(&mut buf, offset) {
            // 如果读取失败，返回非零的操作
            // 如果文件结束，这个错误是 io.EOF
            println!("{:?}", e);
            return Err(e);
        };

        match self.file.read_at(&mut buf, offset) {
            Ok(size) => {
                if size == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "file end"));
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // 将缓冲区buf序列化成Entry
        let mut entry = Entry::decode(buf.as_slice().to_vec());
        // 更新offset
        let mut offset = offset + ENTRY_HEADER_SIZE as u64;

        if entry.key_size > 0 {
            let mut key = vec![0; entry.key_size as usize];

            // 读取 key 到 缓冲区 key中
            if let Err(e) = self.file.read_at(&mut key, offset) {
                return Err(e);
            };

            // 更新key
            entry.key = key;
        }

        // 更新offset
        offset += entry.key_size as u64;
        if entry.value_size > 0 {
            let mut value = vec![0; entry.value_size as usize];
            // 读取 value 到 缓冲区 value中
            if let Err(e) = self.file.read_at(&mut value, offset) {
                return Err(e);
            }
            entry.value = value;
        }

        Ok(entry)
    }

    pub fn write(&mut self, entry: Entry) -> Result<(), io::Error> {
        // 将entry序列化成buf
        let encode = entry.encode();

        // 将buf 写入文件
        if let Err(e) = self.file.write_at(&encode, self.offset) {
            return Err(e);
        }

        // 更新db 文件的 offset
        self.offset += entry.get_size() as u64;

        Ok(())
    }
}

/// 中间函数，创建一个新的db文件
pub fn new_internal(file_name: &str) -> Result<DBFile, io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // 获取文件的大小
    let offset = match file.metadata() {
        Ok(m) => m.len(),
        Err(e) => return Err(e),
    };

    // 创建一个 DBFile 对象
    let db_file = DBFile { file, offset };

    Ok(db_file)
}

/// create a new db file
pub fn new_db_file(path: &str) -> Result<DBFile, io::Error> {
    let file_name = path.to_string() + std::path::MAIN_SEPARATOR.to_string().as_str() + FILE_NAME;
    new_internal(file_name.as_str())
}
