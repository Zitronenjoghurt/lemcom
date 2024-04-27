use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Store {
    pub api_key: String,
}

const FILE_NAME: &str = "store.json";

impl Store {
    pub fn load() -> Result<Self, Error> {
        let path = Path::new(FILE_NAME);

        let mut file = if path.exists() {
            File::open(path)?
        } else {
            File::create(path)?;
            let new_store = Store::default();
            new_store.save()?;
            return Ok(new_store);
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            let new_store = Store::default();
            new_store.save()?;
            return Ok(new_store);
        }
        serde_json::from_str(&contents).map_err(Into::into)
    }

    pub fn save(&self) -> Result<(), Error> {
        let contents = serde_json::to_string(self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE_NAME)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn api_key_available(&self) -> bool {
        return self.api_key.len() > 0;
    }
}
