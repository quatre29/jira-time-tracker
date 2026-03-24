use crate::Result;

#[derive(Clone, Copy)]
pub struct Storage;

impl Storage {
    pub fn load_ticket_keys(&self) -> Result<Vec<String>> {
        let data = match std::fs::read_to_string("src/storage/ticket_keys_storage.json") {
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

        let tmp_path = "src/storage/ticket_keys_storage.tmp";
        let final_path = "src/storage/ticket_keys_storage.json";

        std::fs::write(tmp_path, json)?;
        let _ = std::fs::rename(tmp_path, final_path);

        Ok(())
    }
}
