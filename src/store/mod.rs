use std::{fs::{File, self}, path::Path};

use anyhow::Ok;
use serde::Serialize;

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
}
