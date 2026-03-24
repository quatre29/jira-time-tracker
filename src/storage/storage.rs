use std::{
    env,
    path::{Path, PathBuf},
};

use crate::Result;

#[derive(Clone)]
pub struct Storage {
    ticket_keys_path: PathBuf,
    ticket_keys_tmp_path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let mut ticket_keys_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut ticket_keys_tmp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        ticket_keys_path.push("src/storage/ticket_keys_storage.json");
        ticket_keys_tmp_path.push("src/storage/ticket_keys_storage.tmp");

        Self {
            ticket_keys_path,
            ticket_keys_tmp_path,
        }
    }

    pub fn load_ticket_keys(&self) -> Result<Vec<String>> {
        let data = match std::fs::read_to_string(&self.ticket_keys_path) {
            Ok(key) => key,
            Err(_) => return Ok(vec![]),
        };

        let keys: Vec<String> = serde_json::from_str(&data)?;

        Ok(keys)
    }

    pub fn add_ticket_key(&self, new_key: String) -> Result<()> {
        let mut keys = self.load_ticket_keys()?;
        let new_key = new_key.to_uppercase();

        if !keys.contains(&new_key) {
            keys.push(new_key);
            self.save_ticket_keys(&keys)?;
        }

        Ok(())
    }

    pub fn remove_ticket_key(&self, key: &str) -> Result<()> {
        let mut keys = self.load_ticket_keys()?;

        keys.retain(|k| k != key);

        self.save_ticket_keys(&keys)?;

        Ok(())
    }

    fn save_ticket_keys(&self, keys: &[String]) -> Result<()> {
        let json = serde_json::to_string_pretty(keys)?;

        let tmp_path = &self.ticket_keys_tmp_path;
        let final_path = &self.ticket_keys_path;

        std::fs::write(tmp_path, json)?;
        let _ = std::fs::rename(tmp_path, final_path);

        Ok(())
    }
}
