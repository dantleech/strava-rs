use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub mod activity;

pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub fn new(path: String) -> JsonStorage {
        JsonStorage { path }
    }

    pub fn write<T: Serialize>(&self, name: String, data: T) -> Result<(), anyhow::Error> {
        fs::create_dir_all(&self.path)?;
        let path = Path::new(&self.path).join(name);
        let file: File = File::create(path)?;
        serde_json::to_writer(&file, &data)?;
        Ok(())
    }

    fn load<T: DeserializeOwned>(&self, name: String) -> Vec<T> {
        let path = Path::new(&self.path).join(name);
        let file = File::open(&path).expect(format!("Could not open file: {}", path.display()).as_str());
        let reader = BufReader::new(file);
        let collection: Vec<T> = match serde_json::from_reader(reader) {
            Ok(ok) => ok,
            Err(_err) => Vec::new(),
        };
        collection
    }
}
