use kv::{Bucket, Config, Raw, Store};

pub fn init_new_store(file_path: &str) -> Store {
    let cfg = Config::new(file_path);
    Store::new(cfg).unwrap()
}

pub fn open_bucket(store: Store, bucket_name: &str) -> Bucket<Raw, Raw> {
    let bucket = store.bucket::<Raw, Raw>(Some(bucket_name));
    bucket.unwrap()
}

pub struct EncryptedDb<'a> {
    bucket: Bucket<'a, Raw, Raw>,
}

impl<'a> EncryptedDb<'a> {
    pub fn new(file_path: &str) -> Self {
        let store = init_new_store(file_path);
        let bucket = open_bucket(store, "encrypted db");

        Self { bucket }
    }

    pub fn store(&self, key: &Raw, value: &Raw) {
        self.bucket.set(key, value).unwrap();
    }

    pub fn get(&self, key: &Raw) -> Option<Raw> {
        let val = self.bucket.get(key).unwrap();
        val
    }
}
