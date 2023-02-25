use std::{fs::{File}, path::Path};

use super::{auth_code_fetcher::AuthCodeFetcher, AuthResponse};

pub struct TokenStore {
    path: String,
}

impl TokenStore {
    pub(crate) fn new(path: String) -> Self {
        Self{path}
    }

    pub(crate) fn get(&self) -> Option<AuthResponse> {
        if !Path::new(&self.path).exists() {
            return None
        }
        match File::open(&self.path) {
            Ok(file) => {
                Some(AuthResponse{access_token: "".to_string()})
            }
            Err(e) => None
        }
    }
}
