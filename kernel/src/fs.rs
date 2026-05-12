use alloc::{collections::BTreeMap, string::String, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct RamFs {
    files: BTreeMap<String, Vec<u8>>,
}

impl RamFs {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn write(&mut self, name: &str, data: &[u8]) {
        self.files.insert(name.into(), data.to_vec());
    }

    pub fn read(&self, name: &str) -> Option<Vec<u8>> {
        self.files.get(name).map(|data| data.clone())
    }

    pub fn delete(&mut self, name: &str) -> bool {
        self.files.remove(name).is_some()
    }

    pub fn list(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

lazy_static! {
    static ref RAM_FS: Mutex<RamFs> = Mutex::new(RamFs::new());
}

pub fn init() {
    RAM_FS
        .lock()
        .write("readme.txt", b"duckos ramfs: write <name> <data>, ls, cat, exec");
}

pub fn write(name: &str, data: &[u8]) {
    RAM_FS.lock().write(name, data);
}

pub fn read(name: &str) -> Option<Vec<u8>> {
    RAM_FS.lock().read(name)
}

pub fn delete(name: &str) -> bool {
    RAM_FS.lock().delete(name)
}

pub fn list() -> Vec<String> {
    RAM_FS.lock().list()
}
