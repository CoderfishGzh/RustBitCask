use crate::{
    db_file::{new_db_file, DBFile},
    entry::Entry,
};
use std::collections::HashMap;
use std::io;

pub struct DB {
    pub indexes: HashMap<Vec<u8>, usize>,
    pub db_file: DBFile,
    pub dir_path: &'static str,
}

impl DB {
    pub fn load_indexes_from_file(&mut self) {
        let mut offset: u64 = 0;

        loop {
            let entry = match self.db_file.read(offset as u64) {
                Ok(e) => e,
                Err(err) => {
                    // 读取完毕
                    if err.kind() == io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    return;
                }
            };

            // 设置索引状态
            self.indexes.insert(entry.key.clone(), offset as usize);

            if entry.mark == 0 {
                self.indexes.remove(&entry.key.clone());
            }
            offset += entry.get_size() as u64;
            println!("offset : {}", offset);
        }

        return;
    }

    /// 写入数据
    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), io::Error> {
        if key.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "key is empty"));
        }

        let offset = self.db_file.offset;

        // 将key 和 value 反序列化成entry
        let entry = Entry::new_entry(key.clone(), value, 1);

        self.db_file.write(entry)?;

        self.indexes.insert(key.clone(), offset as usize);

        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> Result<Vec<u8>, io::Error> {
        if key.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "key is empty"));
        }

        // 获取索引信息
        let offset = match self.indexes.get(&key) {
            Some(ot) => ot,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "key not found")),
        };

        // 从磁盘中读取数据
        let entry = match self.db_file.read(offset.clone() as u64) {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        Ok(entry.value)
    }
}

/// Open a database object.
pub fn open(dir_path: &'static str) -> Result<DB, io::Error> {
    // 判断该目录是否存在
    if let Err(_) = std::fs::metadata(dir_path) {
        println!("{} not found", dir_path);
        if let Err(e) = std::fs::create_dir(dir_path) {
            println!("create dir error {}", e);
            // 创建目录失败，返回错误
            return Err(e);
        }
    }

    // 加载数据文件
    let db_file = match new_db_file(dir_path) {
        Ok(f) => f,
        Err(e) => {
            println!("load db file error {}", e);
            return Err(e);
        }
    };
    println!("load db file success");

    // 创建DB对象实例
    let mut db = DB {
        db_file,
        indexes: HashMap::new(),
        dir_path,
    };
    println!("create db success");

    db.load_indexes_from_file();
    println!("load index success");
    return Ok(db);
}
