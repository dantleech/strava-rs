use std::{fs::File, path::Path};

use super::AuthResponse;

pub struct TokenStore {
    path: String,
}

impl TokenStore {
    pub(crate) fn new(path: String) -> Self {
        Self { path }
    }

    pub(crate) fn get(&self) -> Result<Option<AuthResponse>, anyhow::Error> {
        if !Path::new(&self.path).exists() {
            return Ok(None);
        }
        let file = File::open(&self.path)?;
        self.read_file(file)
    }

    fn read_file(&self, file: File) -> Result<Option<AuthResponse>, anyhow::Error> {
        let token: AuthResponse = serde_json::from_reader(&file)?;
        Ok(Some(token))
    }

    pub(crate) fn put(&self, token: &AuthResponse) -> Result<(), anyhow::Error> {
        let file: File = File::create(&self.path)?;
        serde_json::to_writer(&file, token)?;

        Ok(())
    }
}
